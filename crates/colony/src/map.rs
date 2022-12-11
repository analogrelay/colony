use std::io::BufRead;

#[derive(Debug)]
pub struct Tile(usize);

#[derive(Debug)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    tiles: Vec<Tile>,
}

impl Map {
    pub fn get_tile(&self, x: u32, y: u32) -> Option<&Tile> {
        self.tiles.get((y * self.width + x) as usize)
    }

    pub fn read<R: BufRead>(mut reader: R) -> Result<Self, anyhow::Error> {
        let mut header = String::new();
        reader.read_line(&mut header)?;
        let (width, height) = {
            let mut splat = header.split_whitespace();
            let width = splat
                .next()
                .map(|s| s.parse::<usize>())
                .ok_or_else(|| anyhow::Error::msg("invalid map header"))??;
            let height = splat
                .next()
                .map(|s| s.parse::<usize>())
                .ok_or_else(|| anyhow::Error::msg("invalid map header"))??;
            if splat.next().is_some() {
                return Err(anyhow::Error::msg("invalid map header"));
            }
            (width, height)
        };

        // Read the tiles themselves
        let mut tiles = Vec::new();
        let mut lines = reader.lines();
        for _ in 0..height {
            let line = match lines.next() {
                Some(Ok(line)) => line,
                Some(Err(err)) => return Err(err.into()),
                None => return Err(anyhow::Error::msg("too few map lines")),
            };
            let mut splat = line.split_whitespace();
            for _ in 0..width {
                let tile = match splat.next() {
                    Some(tile) => tile,
                    None => return Err(anyhow::Error::msg("map line too short")),
                };
                let tile = tile.parse::<usize>()?;
                tiles.push(Tile(tile));
            }
            if splat.next().is_some() {
                return Err(anyhow::Error::msg("map line too long"));
            }
        }
        if lines.next().is_some() {
            return Err(anyhow::Error::msg("too many map lines"));
        }
        Ok(Map {
            width: width as u32,
            height: height as u32,
            tiles,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let map = Map::read(std::io::Cursor::new(
"2 2
0 1
2 3")).unwrap();
        assert_eq!(map.width, 2);
        assert_eq!(map.height, 2);
        assert_eq!(4, map.tiles.len());
        assert_eq!(map.get_tile(0, 0).unwrap().0, 0);
        assert_eq!(map.get_tile(1, 0).unwrap().0, 1);
        assert_eq!(map.get_tile(0, 1).unwrap().0, 2);
        assert_eq!(map.get_tile(1, 1).unwrap().0, 3);
    }

    #[test]
    fn test_invalid_header() {
        let err = Map::read(std::io::Cursor::new("1")).unwrap_err();
        assert_eq!(err.to_string(), "invalid map header");

        let err = Map::read(std::io::Cursor::new("1 2 3")).unwrap_err();
        assert_eq!(err.to_string(), "invalid map header");

        let err = Map::read(std::io::Cursor::new("1 a")).unwrap_err();
        assert_eq!(err.to_string(), "invalid digit found in string");
    }

    #[test]
    fn test_line_too_long() {
        let err = Map::read(std::io::Cursor::new("1 1
1 2 3")).unwrap_err();
        assert_eq!(err.to_string(), "map line too long");
    }

    #[test]
    fn test_line_too_short() {
        let err = Map::read(std::io::Cursor::new("4 1
1 2 3")).unwrap_err();
        assert_eq!(err.to_string(), "map line too short");
    }

    #[test]
    fn test_too_few_lines() {
        let err = Map::read(std::io::Cursor::new("1 3
1
2")).unwrap_err();
        assert_eq!(err.to_string(), "too few map lines");
    }

    #[test]
    fn test_too_many_lines() {
        let err = Map::read(std::io::Cursor::new("1 1
1
2")).unwrap_err();
        assert_eq!(err.to_string(), "too many map lines");
    }
}