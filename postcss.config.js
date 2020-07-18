const purgecss = require("@fullhuman/postcss-purgecss")({
  content: ["./src/**/*.rs", "./dist/**/*.js", "./static/index.html"],

  defaultExtractor: content => {
    //const broadMatches = content.match(/[^<>"'`\s]*[^<>"'`\s:]/g) || [];
    //const innerMatches = content.match(/[^<>"'`\s.()]*[^<>"'`\s.():]/g) || [];
    //return broadMatches.concat(innerMatches);
    return content.match(/[A-z0-9-_:\/]+/g) || [];
  }
});

module.exports = {
  plugins: [
    require("tailwindcss"),
    require("autoprefixer"),
    ...(process.env.NODE_ENV === "production" ? [purgecss] : [])
  ]
};
