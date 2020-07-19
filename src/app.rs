extern crate js_sys;

use yew::prelude::*;
use yew::services::{IntervalService, Task};
use std::time::Duration;


#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Dead = 0,
    Alive = 1,
}

#[allow(dead_code)]
pub struct Environment {
    active: bool,
    counter: u64,
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    task: Box<dyn Task>,
    link: ComponentLink<Self>,
}

impl Environment {

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn get_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (col + delta_col) % self.width;
                let index = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[index] as u8;
            }
        }
        count
    }

    fn step(&mut self) {
        use Cell::*;
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let neighbours = self.get_neighbour_count(row, col); 

                let next_cell = match(cell, neighbours) {
                    (Alive, 2) | (Alive, 3) => Alive,
                    (Dead, 3) => Alive,
                    (Alive, _) | (Dead, _) => Dead,
                };
                next[index] = next_cell;
            }
        }
        self.counter += 1;
        self.cells = next;
    }

    fn create_random(&mut self) {
        let cells = (0..self.width * self.height)
            .map(|_| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            }).collect();
        self.counter += 1;
        self.cells = cells;
    }
}

pub enum Msg {
    Tick,
    Stop,
    Start,
    Step,
    Random,
}

impl Component for Environment {
    type Message = Msg;
    type Properties = ();

    fn create(_self: Self::Properties, link: ComponentLink<Self>) -> Environment {
        let width = 60;
        let height = 60;

        let callback = link.callback(|_| Msg::Tick);
        let handle = IntervalService::spawn(Duration::from_millis(300), callback);

        Environment {
            active: false,
            counter: 0,
            width,
            height,
            cells: vec![Cell::Dead; (width * height) as usize],
            task: Box::new(handle),
            link: link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Tick => {
                if self.active {
                    self.step()
                }
            },
            Msg::Stop => self.active = false,
            Msg::Start => self.active = true,
            Msg::Step => self.step(),
            Msg::Random => self.create_random(), 
        }
        true
    }


    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ "Game of Life" }</h1>
                <section class="game-of-life",>
                    <div class="board",>
                        { for self.cells.iter().map(display_cell) }
                    </div>
                </section>
                <button 
                    class="button"
                    disabled={ self.active }
                    onclick=self.link.callback(|_| Msg::Random)>{ "Random" }
                </button>
                <button 
                    class="button"
                    disabled={ self.active || self.counter == 0 }
                    onclick=self.link.callback(|_| Msg::Start)>{ "Start" }
                </button>
                <button 
                    class="button"
                    disabled={ !self.active }
                    onclick=self.link.callback(|_| Msg::Stop)>{ "Stop" }
                </button>
                <button 
                    class="button"
                    disabled={ self.active || self.counter == 0 }
                    onclick=self.link.callback(|_| Msg::Step)>{ "Step" }
                </button>
            </div>

        }
    }
}

fn display_cell(cell: &Cell) -> Html {
    let cell_status = if *cell == Cell::Alive { "alive" } else { "dead" };
    html! {
        <div class=("cell", cell_status)></div>
    }
}
