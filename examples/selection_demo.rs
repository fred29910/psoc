//! Selection Tool Demo
//!
//! This example demonstrates the selection tool functionality in PSOC.
//! It shows how to create selections, manipulate them, and integrate with the document system.

use psoc_core::{Document, Point, Selection, SelectionMode};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🎨 PSOC Selection Tool Demo");
    println!("==========================");

    // Create a new document
    let mut document = Document::new("Selection Demo".to_string(), 800, 600);
    println!(
        "📄 Created document: {}x{}",
        document.size.width, document.size.height
    );

    // Initially, there's no selection (select all)
    println!("\n🔍 Initial selection state:");
    println!("  Has selection: {}", document.has_selection());
    println!(
        "  Is select all: {}",
        document.get_selection().is_select_all()
    );

    // Create a rectangular selection
    println!("\n📐 Creating rectangular selection...");
    let selection = Selection::rectangle(100.0, 150.0, 300.0, 200.0);
    document.set_selection(selection);

    println!("  Selection created: {}", document.get_selection());
    println!("  Has selection: {}", document.has_selection());
    println!(
        "  Is select all: {}",
        document.get_selection().is_select_all()
    );

    // Get selection bounds
    if let Some(bounds) = document.selection_bounds() {
        println!(
            "  Selection bounds: x={}, y={}, w={}, h={}",
            bounds.x, bounds.y, bounds.width, bounds.height
        );
        println!(
            "  Selection area: {:.0} pixels",
            document.get_selection().area()
        );
    }

    // Test point selection
    println!("\n🎯 Testing point selection:");
    let test_points = [
        Point::new(250.0, 250.0), // Inside selection
        Point::new(50.0, 100.0),  // Outside selection
        Point::new(100.0, 150.0), // On selection border
        Point::new(400.0, 350.0), // On selection border
    ];

    for (i, point) in test_points.iter().enumerate() {
        let is_selected = document.is_point_selected(*point);
        println!(
            "  Point {}: ({:.0}, {:.0}) -> {}",
            i + 1,
            point.x,
            point.y,
            if is_selected {
                "✅ Selected"
            } else {
                "❌ Not selected"
            }
        );
    }

    // Demonstrate selection from two points (like mouse drag)
    println!("\n🖱️  Creating selection from mouse drag simulation:");
    let start_point = Point::new(200.0, 100.0);
    let end_point = Point::new(500.0, 400.0);

    let drag_selection = Selection::rectangle_from_points(start_point, end_point);
    document.set_selection(drag_selection);

    println!(
        "  Drag from ({:.0}, {:.0}) to ({:.0}, {:.0})",
        start_point.x, start_point.y, end_point.x, end_point.y
    );
    println!("  Resulting selection: {}", document.get_selection());

    // Demonstrate selection modes
    println!("\n🔧 Selection modes:");
    for mode in [
        SelectionMode::Replace,
        SelectionMode::Add,
        SelectionMode::Subtract,
        SelectionMode::Intersect,
    ] {
        println!("  {}: {}", mode, mode);
    }

    // Clear selection
    println!("\n🧹 Clearing selection...");
    document.clear_selection();
    println!("  Has selection: {}", document.has_selection());
    println!(
        "  Is select all: {}",
        document.get_selection().is_select_all()
    );

    // Demonstrate inverted selection
    println!("\n🔄 Creating inverted selection:");
    use psoc_core::RectangleSelection;
    let inverted_rect = RectangleSelection::new_inverted(200.0, 200.0, 100.0, 100.0);
    let inverted_selection = Selection::Rectangle(inverted_rect);
    document.set_selection(inverted_selection);

    println!("  Inverted selection: {}", document.get_selection());

    // Test points with inverted selection
    let test_point_inside = Point::new(250.0, 250.0); // Inside the rectangle
    let test_point_outside = Point::new(100.0, 100.0); // Outside the rectangle

    println!(
        "  Point inside rect ({:.0}, {:.0}): {}",
        test_point_inside.x,
        test_point_inside.y,
        if document.is_point_selected(test_point_inside) {
            "✅ Selected"
        } else {
            "❌ Not selected"
        }
    );
    println!(
        "  Point outside rect ({:.0}, {:.0}): {}",
        test_point_outside.x,
        test_point_outside.y,
        if document.is_point_selected(test_point_outside) {
            "✅ Selected"
        } else {
            "❌ Not selected"
        }
    );

    // Demonstrate selection transformation
    println!("\n🔄 Selection transformations:");
    let mut transform_selection = Selection::rectangle(100.0, 100.0, 200.0, 150.0);
    println!("  Original: {}", transform_selection);

    // Translate
    transform_selection.translate(50.0, 25.0);
    println!("  After translate(50, 25): {}", transform_selection);

    // Scale
    transform_selection.scale(1.5);
    println!("  After scale(1.5): {}", transform_selection);

    println!("\n✨ Selection demo completed successfully!");
    println!("   This demonstrates the core selection functionality that enables");
    println!("   precise editing operations in PSOC image editor.");

    Ok(())
}
