//
// This is only a SKELETON file for the 'Zipper' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export class Zipper {
  constructor(tree, focus) {
    this.tree = tree;
    this.focus = focus || tree
  }

  static fromTree(orig_tree, focus, val) {
    if (val === null) {
      return null;
    }
    return new Zipper(orig_tree, focus);
  }

  toTree() {
    return this.tree;
  }

  value() {
    return this.focus.value;
  }

  left() {
    return Zipper.fromTree(this.tree, this.focus, this.focus.left);
  }

  right() {
    return Zipper.fromTree(this.tree, this.focus, this.focus.right);
  }

  up() {
    return this.focus

  }

  setValue() {
    throw new Error("Remove this statement and implement this function");
  }

  setLeft() {
    throw new Error("Remove this statement and implement this function");
  }

  setRight() {
    throw new Error("Remove this statement and implement this function");
  }
}
