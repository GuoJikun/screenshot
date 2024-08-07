# Screenshot_desktop

Take a screenshot of a screen;

![Crates.io License](https://img.shields.io/crates/l/screenshot_desktop)
![Crates.io Version](https://img.shields.io/crates/v/screenshot_desktop)

## Usage

First, add screenshot to your project by cargo.

```rs
use std::path::Path;
use screenshot_desktop::Screenshot;

fn main(){
    let _ = match Screenshot::new() {
        Ok(screenshot) => {
            screenshot.save(&Path::new("screenshot.png")).map_err(|e| e.to_string())
        },
        Err(_) => {
            Err("Failed to capture screenshot".to_string())
        },
    };
}
```

## Supported Platform

| Platform/平台 | status/状态 |
| ------------- | ----------- |
| Windows       | Done        |
| Linux         | Come Soon   |
| Macos         | Come Soon   |

## License

MIT + Apache-2.0