use calamine::{open_workbook, DataType, Reader, Xlsx, XlsxError};// Ensure required modules are imported

/// Represents a single game release with platform and release year.
pub struct GameRelease {
    pub platform: String,
    pub year: u32,
}

/// Reads the Excel file and returns a vector of game releases.
pub fn read_game_data(file_path: &str) -> Result<Vec<GameRelease>, String> {
    let mut workbook: Xlsx<_> = open_workbook(file_path)
        .map_err(|e: XlsxError| e.to_string())?;  // Add explicit type annotation here
    let range = workbook.worksheet_range("Sheet1")
        .map_err(|e: XlsxError| format!("Failed to read sheet: {}", e))?;  // Add explicit type annotation here

    let mut game_releases = Vec::new();
    for row in range.rows().skip(1) { // Skip the header row
        let platform = row[0].get_string().unwrap_or_default().to_string();
        let year = row[1].get_float().ok_or("Year must be a number")? as u32;

        game_releases.push(GameRelease {
            platform,
            year,
        });
    }
    Ok(game_releases)
    
}

/// exam function to show how data could be used
pub fn data_usage() {
    let file_path = "data/statistic_id1391589_number-of-major-upcoming-video-games-2022-2024-by-platform.xlsx";
    match read_game_data(file_path) {
        Ok(game_releases) => {
            for game in game_releases {
                println!("Platform: {}, Year: {}", game.platform, game.year);
            }
        }
        Err(e) => {
            println!("Failed to read game data: {}", e);
        }
    }
}
