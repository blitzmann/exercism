const COLORS = [
  "black",
  "brown",
  "red",
  "orange",
  "yellow",
  "green",
  "blue",
  "violet",
  "grey",
  "white"
];

export class ResistorColor {
  constructor(private colors: string[]) {
    if (colors.length < 2) {
      throw Error("At least two colors need to be present");
    }
    this.colors = colors.slice(0, 2);
  }
  value() {
    let output = "";
    for (let color of this.colors) {
      output += COLORS.indexOf(color);
    }
    return +output;
  }
}
