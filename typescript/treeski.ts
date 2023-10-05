function getInput(): string {
    return `..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#`;
}

enum Icon {
    Tree,
    Snow,
}

const icons = getInput()
    .split("\n")
    .map(x => x.split("").map(x => x === "." ? Icon.Snow : Icon.Tree));

const rowLen = icons[0].length;
let treeCount = 0;

icons.forEach((row, pos) => {
    if (row[pos * 3 % rowLen] === Icon.Tree) {
        treeCount++;
    }
});

console.log("TreeCount", treeCount);
