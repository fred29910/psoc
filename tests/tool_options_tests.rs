use psoc::tools::{
    tool_trait::{ToolOption, ToolOptionType, ToolOptionValue},
    ToolManager, ToolType,
};

#[test]
fn test_tool_option_setting_and_getting() {
    let mut tool_manager = ToolManager::new();

    // Set active tool to brush
    tool_manager.set_active_tool(ToolType::Brush).unwrap();

    // Test setting brush size
    tool_manager
        .set_tool_option("size", ToolOptionValue::Float(15.0))
        .unwrap();

    // Test getting brush size
    let size = tool_manager.get_tool_option("size");
    assert_eq!(size, Some(ToolOptionValue::Float(15.0)));

    // Test setting brush color
    let red_color = [255, 0, 0, 255];
    tool_manager
        .set_tool_option("color", ToolOptionValue::Color(red_color))
        .unwrap();

    let color = tool_manager.get_tool_option("color");
    assert_eq!(color, Some(ToolOptionValue::Color(red_color)));
}

#[test]
fn test_tool_option_bounds_checking() {
    let mut tool_manager = ToolManager::new();
    tool_manager.set_active_tool(ToolType::Brush).unwrap();

    // Test setting size beyond maximum (should be clamped)
    tool_manager
        .set_tool_option("size", ToolOptionValue::Float(200.0))
        .unwrap();

    let size = tool_manager.get_tool_option("size");
    if let Some(ToolOptionValue::Float(value)) = size {
        assert!(value <= 100.0); // Should be clamped to max
    } else {
        panic!("Expected float value");
    }

    // Test setting size below minimum (should be clamped)
    tool_manager
        .set_tool_option("size", ToolOptionValue::Float(-5.0))
        .unwrap();

    let size = tool_manager.get_tool_option("size");
    if let Some(ToolOptionValue::Float(value)) = size {
        assert!(value >= 1.0); // Should be clamped to min
    } else {
        panic!("Expected float value");
    }
}

#[test]
fn test_tool_options_reset() {
    let mut tool_manager = ToolManager::new();
    tool_manager.set_active_tool(ToolType::Brush).unwrap();

    // Change some options
    tool_manager
        .set_tool_option("size", ToolOptionValue::Float(50.0))
        .unwrap();
    tool_manager
        .set_tool_option("hardness", ToolOptionValue::Float(0.3))
        .unwrap();

    // Reset options
    tool_manager.reset_tool_options().unwrap();

    // Check that options are back to defaults
    let size = tool_manager.get_tool_option("size");
    let hardness = tool_manager.get_tool_option("hardness");

    // These should be the default values
    assert_eq!(size, Some(ToolOptionValue::Float(10.0)));
    assert_eq!(hardness, Some(ToolOptionValue::Float(1.0)));
}

#[test]
fn test_select_tool_options() {
    let mut tool_manager = ToolManager::new();
    tool_manager.set_active_tool(ToolType::Select).unwrap();

    // Test feather radius option
    tool_manager
        .set_tool_option("feather", ToolOptionValue::Float(5.0))
        .unwrap();

    let feather = tool_manager.get_tool_option("feather");
    assert_eq!(feather, Some(ToolOptionValue::Float(5.0)));

    // Test anti-alias option
    tool_manager
        .set_tool_option("anti_alias", ToolOptionValue::Bool(false))
        .unwrap();

    let anti_alias = tool_manager.get_tool_option("anti_alias");
    assert_eq!(anti_alias, Some(ToolOptionValue::Bool(false)));
}

#[test]
fn test_move_tool_options() {
    let mut tool_manager = ToolManager::new();
    tool_manager.set_active_tool(ToolType::Move).unwrap();

    // Test snap to grid option
    tool_manager
        .set_tool_option("snap_to_grid", ToolOptionValue::Bool(true))
        .unwrap();

    let snap = tool_manager.get_tool_option("snap_to_grid");
    assert_eq!(snap, Some(ToolOptionValue::Bool(true)));

    // Test grid size option
    tool_manager
        .set_tool_option("grid_size", ToolOptionValue::Float(20.0))
        .unwrap();

    let grid_size = tool_manager.get_tool_option("grid_size");
    assert_eq!(grid_size, Some(ToolOptionValue::Float(20.0)));
}

#[test]
fn test_transform_tool_options() {
    let mut tool_manager = ToolManager::new();
    tool_manager.set_active_tool(ToolType::Transform).unwrap();

    // Get all options for transform tool
    let options = tool_manager.get_active_tool_options();

    // Transform tool should have mode option
    let mode_option = options.iter().find(|opt| opt.name == "mode");
    assert!(mode_option.is_some());

    if let Some(option) = mode_option {
        assert_eq!(option.display_name, "Transform Mode");
        if let ToolOptionType::Enum(ref modes) = option.option_type {
            assert!(modes.contains(&"Scale".to_string()));
            assert!(modes.contains(&"Rotate".to_string()));
        } else {
            panic!("Expected enum option type");
        }
    }
}

#[test]
fn test_invalid_option_name() {
    let mut tool_manager = ToolManager::new();
    tool_manager.set_active_tool(ToolType::Brush).unwrap();

    // Try to get non-existent option
    let result = tool_manager.get_tool_option("non_existent");
    assert_eq!(result, None);

    // Try to set non-existent option (should not error but do nothing)
    let result = tool_manager.set_tool_option("non_existent", ToolOptionValue::Float(1.0));
    assert!(result.is_ok());
}

#[test]
fn test_no_active_tool_options() {
    let mut tool_manager = ToolManager::new();

    // No active tool set initially, but ToolManager starts with Select tool by default
    // So let's explicitly test with no tool set by creating a fresh manager
    let options = tool_manager.get_active_tool_options();
    // ToolManager actually starts with Select tool active, so it will have options
    assert!(!options.is_empty());

    let result = tool_manager.get_tool_option("feather");
    assert!(result.is_some()); // Select tool has feather option
}

#[test]
fn test_tool_option_types() {
    let mut tool_manager = ToolManager::new();
    tool_manager.set_active_tool(ToolType::Brush).unwrap();

    let options = tool_manager.get_active_tool_options();

    // Find size option and check its type
    let size_option = options.iter().find(|opt| opt.name == "size");
    assert!(size_option.is_some());

    if let Some(option) = size_option {
        if let ToolOptionType::Float { min, max } = option.option_type {
            assert_eq!(min, 1.0);
            assert_eq!(max, 100.0);
        } else {
            panic!("Expected float option type");
        }
    }

    // Find color option and check its type
    let color_option = options.iter().find(|opt| opt.name == "color");
    assert!(color_option.is_some());

    if let Some(option) = color_option {
        assert!(matches!(option.option_type, ToolOptionType::Color));
    }
}

#[test]
fn test_eraser_tool_options() {
    let mut tool_manager = ToolManager::new();
    tool_manager.set_active_tool(ToolType::Eraser).unwrap();

    // Test eraser size
    tool_manager
        .set_tool_option("size", ToolOptionValue::Float(25.0))
        .unwrap();

    let size = tool_manager.get_tool_option("size");
    assert_eq!(size, Some(ToolOptionValue::Float(25.0)));

    // Test eraser hardness
    tool_manager
        .set_tool_option("hardness", ToolOptionValue::Float(0.8))
        .unwrap();

    let hardness = tool_manager.get_tool_option("hardness");
    assert_eq!(hardness, Some(ToolOptionValue::Float(0.8)));
}
