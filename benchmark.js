const puppeteer = require("puppeteer");

const SECONDES = 1000;

(async () => {
  const browser = await puppeteer.launch({ headless: false });
  const page = await browser.newPage();
  await page.goto("http://localhost:5000");
  await page.evaluate(index => {
    let cells = Array.from(document.querySelectorAll(".cell"));
    function getRand(min, max) {
      min = Math.ceil(min);
      max = Math.floor(max);
      return Math.floor(Math.random() * (max - min + 1)) + min;
    }
    cells = [...cells].filter(node => node.className === "cell");
    cells[getRand(0, 9)].click();

    cells = [...cells].filter(node => node.className === "cell");
    cells[getRand(0, 7)].click();

    cells = [...cells].filter(node => node.className === "cell");
    cells[getRand(0, 5)].click();

    cells = [...cells].filter(node => node.className === "cell");
    cells[getRand(0, 3)].click();

    cells = [...cells].filter(node => node.className === "cell");
    cells[getRand(0, 1)].click();
  });
})();
