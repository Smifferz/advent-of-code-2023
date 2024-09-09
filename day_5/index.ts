interface IMap {
  source: string;
  destination: string;
  ranges: IRange[];
}

interface IRange {
  source_start: number;
  destination_start: number;
  length: number;
}

function parse_part1_seeds(input: string): number[] {
  let seeds: number[] = [];
  let current_seed: string = "";

  // Assume the seed description is on the first line.
  // NOTE: If it isn't this will break!'
  for (let c of input) {
    // If this is a number then parse this as the current seed
    if (c >= "0" && c <= "9") {
      current_seed += c;
    } else {
      // If there is a seed set then parse it and reset it
      if (current_seed != "") {
        seeds.push(parseInt(current_seed));
        current_seed = "";
      }
    }
  }

  // Add the remaining seed
  if (current_seed != "") {
    seeds.push(parseInt(current_seed));
  }

  return seeds;
}

function parse_part2_seeds(input: string): number[] {
  let seeds: number[] = [];
  let current_seed: string = "";
  let current_range: string = "";
  let seed_switch = 0;

  // Assume the seed description is on the first line.
  // NOTE: If it isn't this will break!'
  for (let c of input) {
    // If this is a number then parse this as the current seed
    if (c >= "0" && c <= "9") {
      if (seed_switch == 0) {
        current_seed += c;
      } else {
        current_range += c;
      }
    } else {
      // If there is a seed set then parse it and reset it
      if (seed_switch == 0 && current_seed != "") {
        seed_switch = 1;
      } else if (seed_switch == 1 && current_range != "") {
        for (
          let i = parseInt(current_seed);
          i < parseInt(current_seed) + parseInt(current_range);
          i++
        ) {
          seeds.push(i);
        }
        current_seed = "";
        current_range = "";
        seed_switch = 0;
      }
    }
  }

  // Add the remaining seed
  if (current_seed != "") {
    if (seed_switch == 0 && current_seed != "") {
      seed_switch = 1;
    } else if (seed_switch == 1 && current_range != "") {
      for (
        let i = parseInt(current_seed);
        i < parseInt(current_seed) + parseInt(current_range);
        i++
      ) {
        seeds.push(i);
      }
      current_seed = "";
      current_range = "";
      seed_switch = 0;
    }
  }

  return seeds;
}

export function calc_part1(input: string): number {
  let lowest_location = Infinity;

  // Parse the seeds from the first line of the input
  let lines = input.split("\n");

  let seeds = parse_part1_seeds(lines[0]);

  console.log(JSON.stringify(seeds));

  // Parse the maps
  let maps: IMap[] = [];
  let map: IMap | undefined = undefined;
  let map_idx = 0;
  for (let i = 1; i < lines.length; i++) {
    if (lines[i].trim() == "") {
      // If the map is defined then add it to the total maps
      if (map != undefined) {
        maps.push(map);
        map = undefined;
      }

      map_idx = 0;

      continue;
    }

    switch (map_idx) {
      case 0:
        // Parse the source->destination map
        let source_dest_regex = /(?<source>\w+)-to-(?<destination>\w+) map/g;
        let matches = source_dest_regex.exec(lines[i]);

        if (matches?.groups) {
          map = {
            source: matches?.groups["source"],
            destination: matches?.groups["destination"],
            ranges: [],
          };
        }
        map_idx = 1;
        break;
      case 1:
        let range_components_re = /(\d+)/g;
        let range_components = [...lines[i].matchAll(range_components_re)];
        let range: IRange = {
          source_start: parseInt(range_components[1][0]),
          destination_start: parseInt(range_components[0][0]),
          length: parseInt(range_components[2][0]),
        };
        map?.ranges.push(range);
        break;
      default:
        console.error("Encountered an invalid seed map index");
    }
  }

  // TODO (alex) Traverse the maps to find the lowest location
  seeds.forEach((seed) => {
    // console.log("Testing seed: " + seed);
    let category = "seed";
    let current_value = seed;
    while (category != "location") {
      let target_map = maps.find((x) => x.source == category);
      category = target_map!.destination;
      target_map!.ranges.some((range) => {
        // The value is within the current range
        if (
          current_value >= range.source_start &&
          current_value <= range.source_start + range.length
        ) {
          current_value =
            current_value - range.source_start + range.destination_start;
          return true;
        }
      });
      // console.log(`Category ${category} matched to value ${current_value}`);
    }
    // console.log("Location = " + current_value);
    if (current_value < lowest_location) {
      lowest_location = current_value;
    }
  });

  return lowest_location;
}

export function calc_part2(input: string): number {
  let lowest_location = Infinity;

  // Parse the seeds from the first line of the input
  let lines = input.split("\n");

  let seeds = parse_part2_seeds(lines[0]);

  console.log(JSON.stringify(seeds));

  // Parse the maps
  let maps: IMap[] = [];
  let map: IMap | undefined = undefined;
  let map_idx = 0;
  for (let i = 1; i < lines.length; i++) {
    if (lines[i].trim() == "") {
      // If the map is defined then add it to the total maps
      if (map != undefined) {
        maps.push(map);
        map = undefined;
      }

      map_idx = 0;

      continue;
    }

    switch (map_idx) {
      case 0:
        // Parse the source->destination map
        let source_dest_regex = /(?<source>\w+)-to-(?<destination>\w+) map/g;
        let matches = source_dest_regex.exec(lines[i]);

        if (matches?.groups) {
          map = {
            source: matches?.groups["source"],
            destination: matches?.groups["destination"],
            ranges: [],
          };
        }
        map_idx = 1;
        break;
      case 1:
        let range_components_re = /(\d+)/g;
        let range_components = [...lines[i].matchAll(range_components_re)];
        let range: IRange = {
          source_start: parseInt(range_components[1][0]),
          destination_start: parseInt(range_components[0][0]),
          length: parseInt(range_components[2][0]),
        };
        map?.ranges.push(range);
        break;
      default:
        console.error("Encountered an invalid seed map index");
    }
  }

  seeds.forEach((seed) => {
    // console.log("Testing seed: " + seed);
    let category = "seed";
    let current_value = seed;
    while (category != "location") {
      let target_map = maps.find((x) => x.source == category);
      category = target_map!.destination;
      target_map!.ranges.some((range) => {
        // The value is within the current range
        if (
          current_value >= range.source_start &&
          current_value <= range.source_start + range.length
        ) {
          current_value =
            current_value - range.source_start + range.destination_start;
          return true;
        }
      });
      // console.log(`Category ${category} matched to value ${current_value}`);
    }
    // console.log("Location = " + current_value);
    if (current_value < lowest_location) {
      lowest_location = current_value;
    }
  });

  return lowest_location;
}

// Main should parse the input files and print the results
async function main() {
  const input_path = "./resources/input.txt";
  const input_file = Bun.file(input_path);
  const input = await input_file.text();

  let part1 = calc_part1(input);
  console.log("Output of part1 = " + part1);

  let part2 = calc_part2(input);
  console.log("Output of part2 = " + part2);
}

if (process.argv.length > 2) {
  await main();
}
