
use super::{generate_roof, generate_output_grid, generate_base, smooth_base};

/////////////////////////////////
// Grid generation tests
/////////////////////////////////

#[test]
// *
//
//   A
//  / \
// +---+
// |   |
// +---+
fn generate_output_grid_single() {
    let mut v = vec![];
    v.push("*".chars().collect());
    let (_, result) = generate_output_grid(&v);

    let mut expected: Vec<Vec<char>> = vec![];
    // for the roof
    expected.push("     ".chars().collect());
    expected.push("     ".chars().collect());
    // for the base
    expected.push("     ".chars().collect());
    expected.push("     ".chars().collect());
    expected.push("     ".chars().collect());
    assert_eq!(result, expected);
}

#[test]
fn generate_output_grid_5by5() {
    let mut v = vec![];
    v.push("    *".chars().collect());
    v.push("*** *".chars().collect());
    v.push("*** *".chars().collect());
    v.push("*****".chars().collect());
    v.push("*****".chars().collect());
    let (_, result) = generate_output_grid(&v);

    let mut expected: Vec<Vec<char>> = vec![];
    // for the roof
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    // for the base
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    assert_eq!(result, expected);
}


// *****
//
//           A
//          / \
//         /   \
//        /     \
//       /       \
//      /         \
//     /           \
//    /             \
//   /               \
//  /                 \
// +-------------------+
// |                   |
// +-------------------+

#[test]
fn generate_output_grid_1by5() {
    let mut v = vec![];
    v.push("*****".chars().collect());
    let (_, result) = generate_output_grid(&v);

    let mut expected: Vec<Vec<char>> = vec![];    // for the roof
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    // for the base
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    assert_eq!(result, expected);
}


/////////////////////////////////
// Roof tests
/////////////////////////////////
#[test]
fn generate_roof_basic() {
    let mut v = vec![];
    v.push("*".chars().collect());

    let (roof_size, mut  output) = generate_output_grid(&v);

    generate_roof(&v, roof_size, &mut output);

    let mut expected: Vec<Vec<char>> = vec![];
    // for the roof
    expected.push(r"  A  ".chars().collect());
    expected.push(r" / \ ".chars().collect());
    // for the base
    expected.push("     ".chars().collect());
    expected.push("     ".chars().collect());
    expected.push("     ".chars().collect());
    assert_eq!(output, expected);
}

#[test]
fn generate_roof_grid_1by5() {
    let mut v = vec![];
    v.push("*****".chars().collect());

    let (roof_size, mut  output) = generate_output_grid(&v);

    generate_roof(&v, roof_size, &mut output);

    let mut expected: Vec<Vec<char>> = vec![];    // for the roof
    expected.push(r"          A          ".chars().collect());
    expected.push(r"         / \         ".chars().collect());
    expected.push(r"        /   \        ".chars().collect());
    expected.push(r"       /     \       ".chars().collect());
    expected.push(r"      /       \      ".chars().collect());
    expected.push(r"     /         \     ".chars().collect());
    expected.push(r"    /           \    ".chars().collect());
    expected.push(r"   /             \   ".chars().collect());
    expected.push(r"  /               \  ".chars().collect());
    expected.push(r" /                 \ ".chars().collect());
    // for the base
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    expected.push("                     ".chars().collect());
    assert_eq!(output, expected);
}

#[test]
fn generate_roof_grid_5by5() {
    let mut v = vec![];
    v.push("    *".chars().collect());
    v.push("*** *".chars().collect());
    v.push("*** *".chars().collect());
    v.push("*****".chars().collect());
    v.push("*****".chars().collect());

    let (roof_size, mut  output) = generate_output_grid(&v);

    generate_roof(&v, roof_size, &mut output);

    let mut expected: Vec<Vec<char>> = vec![];
    // for the roof
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"      A              ".chars().collect());
    expected.push(r"     / \             ".chars().collect());
    expected.push(r"    /   \         A  ".chars().collect());
    expected.push(r"   /     \       / \ ".chars().collect());
    // for the base
    expected.push(r"  /       \          ".chars().collect());
    expected.push(r" /         \         ".chars().collect());
    expected.push( "                     ".chars().collect());
    expected.push( "                     ".chars().collect());
    expected.push( "              A      ".chars().collect());
    expected.push(r"             / \     ".chars().collect());
    expected.push( "                     ".chars().collect());
    expected.push( "                     ".chars().collect());
    expected.push( "                     ".chars().collect());
    expected.push( "                     ".chars().collect());
    expected.push( "                     ".chars().collect());
    assert_eq!(output, expected);
}

    // expected.push(r"                     ".chars().collect());
    // expected.push(r"                     ".chars().collect());
    // expected.push(r"                     ".chars().collect());
    // expected.push(r"                     ".chars().collect());
    // expected.push(r"                     ".chars().collect());
    // expected.push(r"                     ".chars().collect());
    // expected.push(r"      A              ".chars().collect());
    // expected.push(r"     / \             ".chars().collect());
    // expected.push(r"    /   \         A  ".chars().collect());
    // expected.push(r"   /     \       / \ ".chars().collect());
    // // for the base
    // expected.push(r"  /       \     +---+".chars().collect());
    // expected.push(r" /         \    |   |".chars().collect());
    // expected.push( "+---+---+---+   +---+".chars().collect());
    // expected.push( "|   |   |   |   |   |".chars().collect());
    // expected.push( "+---+---+---+ A +---+".chars().collect());
    // expected.push(r"|   |   |   |/ \|   |".chars().collect());
    // expected.push( "+---+---+---+---+---+".chars().collect());
    // expected.push( "|   |   |   |   |   |".chars().collect());
    // expected.push( "+---+---+---+---+---+".chars().collect());
    // expected.push( "|   |   |   |   |   |".chars().collect());
    // expected.push( "+---+---+---+---+---+".chars().collect());

/////////////////////////////////
// Base tests
/////////////////////////////////
#[test]
fn generate_base_basic() {
    let mut v = vec![];
    v.push("*".chars().collect());

    let (_, mut  output) = generate_output_grid(&v);

    generate_base(&v, &mut output);

    let mut expected: Vec<Vec<char>> = vec![];
    // for the roof
    expected.push(r"     ".chars().collect());
    expected.push(r"     ".chars().collect());
    // for the base
    expected.push("+---+".chars().collect());
    expected.push("|   |".chars().collect());
    expected.push("+---+".chars().collect());
    assert_eq!(output, expected);
}

#[test]
fn generate_base_grid_1by5() {
    let mut v = vec![];
    v.push("*****".chars().collect());

    let (_, mut  output) = generate_output_grid(&v);

    generate_base(&v, &mut output);

    let mut expected: Vec<Vec<char>> = vec![];    // for the roof
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    // for the base
    expected.push(r"+---+---+---+---+---+".chars().collect());
    expected.push(r"|   |   |   |   |   |".chars().collect());
    expected.push(r"+---+---+---+---+---+".chars().collect());
    assert_eq!(output, expected);
}

#[test]
fn generate_base_grid_5by5() {
    let mut v = vec![];
    v.push("    *".chars().collect());
    v.push("*** *".chars().collect());
    v.push("*** *".chars().collect());
    v.push("*****".chars().collect());
    v.push("*****".chars().collect());

    let (_, mut  output) = generate_output_grid(&v);

    generate_base(&v, &mut output);

    let mut expected: Vec<Vec<char>> = vec![];
    // for the roof
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    // for the base
    expected.push(r"                +---+".chars().collect());
    expected.push(r"                |   |".chars().collect());
    expected.push( "+---+---+---+   +---+".chars().collect());
    expected.push( "|   |   |   |   |   |".chars().collect());
    expected.push( "+---+---+---+   +---+".chars().collect());
    expected.push(r"|   |   |   |   |   |".chars().collect());
    expected.push( "+---+---+---+---+---+".chars().collect());
    expected.push( "|   |   |   |   |   |".chars().collect());
    expected.push( "+---+---+---+---+---+".chars().collect());
    expected.push( "|   |   |   |   |   |".chars().collect());
    expected.push( "+---+---+---+---+---+".chars().collect());
    assert_eq!(output, expected);
}


/////////////////////////////////
// Smooth tests
/////////////////////////////////
#[test]
fn smooth_base_basic() {
    let mut v = vec![];
    v.push("*".chars().collect());

    let (_, mut  output) = generate_output_grid(&v);

    generate_base(&v, &mut output);
    smooth_base(&mut output);

    let mut expected: Vec<Vec<char>> = vec![];
    // for the roof
    expected.push(r"     ".chars().collect());
    expected.push(r"     ".chars().collect());
    // for the base
    expected.push("+---+".chars().collect());
    expected.push("|   |".chars().collect());
    expected.push("+---+".chars().collect());
    assert_eq!(output, expected);
}

#[test]
fn smooth_base_grid_1by5() {
    let mut v = vec![];
    v.push("*****".chars().collect());

    let (_, mut  output) = generate_output_grid(&v);

    generate_base(&v, &mut output);
    smooth_base(&mut output);

    let mut expected: Vec<Vec<char>> = vec![];    // for the roof
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    // for the base
    expected.push(r"+-------------------+".chars().collect());
    expected.push(r"|                   |".chars().collect());
    expected.push(r"+-------------------+".chars().collect());
    assert_eq!(output, expected);
}

#[test]
fn smooth_base_grid_5by5() {
    let mut v = vec![];
    v.push("    *".chars().collect());
    v.push("*** *".chars().collect());
    v.push("*** *".chars().collect());
    v.push("*****".chars().collect());
    v.push("*****".chars().collect());

    let (_, mut  output) = generate_output_grid(&v);

    generate_base(&v, &mut output);
    smooth_base(&mut output);

    let mut expected: Vec<Vec<char>> = vec![];
    // for the roof
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    expected.push(r"                     ".chars().collect());
    // for the base
    expected.push(r"                +---+".chars().collect());
    expected.push(r"                |   |".chars().collect());
    expected.push( "+-----------+   |   |".chars().collect());
    expected.push( "|           |   |   |".chars().collect());
    expected.push( "|           |   |   |".chars().collect());
    expected.push(r"|           |   |   |".chars().collect());
    expected.push( "|           +---+   |".chars().collect());
    expected.push( "|                   |".chars().collect());
    expected.push( "|                   |".chars().collect());
    expected.push( "|                   |".chars().collect());
    expected.push( "+-------------------+".chars().collect());
    assert_eq!(output, expected);
}
