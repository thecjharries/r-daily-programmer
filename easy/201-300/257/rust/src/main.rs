// Copyright 2022 CJ Harries
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref YEAR_PATTERN: Regex = Regex::new(r"(?U).*(\d{4}).*(\d{4}|,\s*,).*").unwrap();
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_max_presidential_years(input: &str) -> Vec<u32> {
    let mut years: HashMap<u32, u32> = HashMap::new();
    let mut max_count = 0;
    for line in input.lines() {
        if let Some(captures) = YEAR_PATTERN.captures(line) {
            let start_year = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let end_year = captures
                .get(2)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap_or(2016);
            for year in start_year..=end_year {
                *years.entry(year).or_insert(0) += 1;
                if years[&year] > max_count {
                    max_count = years[&year];
                }
            }
        }
    }
    let mut result = years
        .iter()
        .filter(|(_, count)| *count == &max_count)
        .map(|(year, _)| *year)
        .collect::<Vec<u32>>();
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_presidential_years() {
        assert_eq!(vec![1822, 1823, 1824, 1825, 1826, 1831, 1833, 1834, 1835, 1836, 1837, 1838, 1839, 1840, 1841, 1843, 1844, 1845], find_max_presidential_years("    George Washington,	Feb 22 1732,	Westmoreland Co. Va.,	Dec 14 1799,	Mount Vernon Va.
    John Adams,	Oct 30 1735,	Quincy Mass.,	July 4 1826,	Quincy Mass.
    Thomas Jefferson,	Apr 13 1743,	Albemarle Co. Va.,	July 4 1826,	Albemarle Co. Va.
    James Madison,	Mar 16 1751,	Port Conway Va.,	June 28 1836,	Orange Co. Va.
    James Monroe,	Apr 28 1758,	Westmoreland Co. Va.,	July 4 1831,	New York New York
    John Quincy Adams,	July 11 1767,	Quincy Mass.,	Feb 23 1848,	Washington D.C.
    Andrew Jackson,	Mar 15 1767,	Lancaster Co. S.C.,	June 8 1845,	Nashville Tennessee
    Martin Van Buren,	Dec 5 1782,	Kinderhook New York,	July 24 1862,	Kinderhook New York
    William Henry Harrison,	Feb 9 1773,	Charles City Co. Va.,	Apr 4 1841,	Washington D.C.
    John Tyler,	Mar 29 1790,	Charles City Co. Va.,	Jan 18 1862,	Richmond Va.
    James K. Polk,	Nov 2 1795,	Mecklenburg Co. N.C.,	June 15 1849,	Nashville Tennessee
    Zachary Taylor,	Nov 24 1784,	Orange County Va.,	July 9 1850,	Washington D.C
    Millard Fillmore,	Jan 7 1800,	Cayuga Co. New York,	Mar 8 1874,	Buffalo New York
    Franklin Pierce,	Nov 23 1804,	Hillsborough N.H.,	Oct 8 1869,	Concord New Hamp.
    James Buchanan,	Apr 23 1791,	Cove Gap Pa.,	June 1 1868,	Lancaster Pa.
    Abraham Lincoln,	Feb 12 1809,	LaRue Co. Kentucky,	Apr 15 1865,	Washington D.C.
    Andrew Johnson,	Dec 29 1808,	Raleigh North Carolina,	July 31 1875,	Elizabethton Tenn.
    Ulysses S. Grant,	Apr 27 1822,	Point Pleasant Ohio,	July 23 1885,	Wilton New York
    Rutherford B. Hayes,	Oct 4 1822,	Delaware Ohio,	Jan 17 1893,	Fremont Ohio
    James A. Garfield,	Nov 19 1831,	Cuyahoga Co. Ohio,	Sep 19 1881,	Elberon New Jersey
    Chester Arthur,	Oct 5 1829,	Fairfield Vermont,	Nov 18 1886,	New York New York
    Grover Cleveland,	Mar 18 1837,	Caldwell New Jersey,	June 24 1908,	Princeton New Jersey
    Benjamin Harrison,	Aug 20 1833,	North Bend Ohio,	Mar 13 1901,	Indianapolis Indiana
    William McKinley,	Jan 29 1843,	Niles Ohio,	Sep 14 1901,	Buffalo New York
    Theodore Roosevelt,	Oct 27 1858,	New York New York,	Jan 6 1919,	Oyster Bay New York
    William Howard Taft,	Sep 15 1857,	Cincinnati Ohio,	Mar 8 1930,	Washington D.C.
    Woodrow Wilson,	Dec 28 1856,	Staunton Virginia,	Feb 3 1924,	Washington D.C.
    Warren G. Harding,	Nov 2 1865,	Morrow County Ohio,	Aug 2 1923,	San Francisco Cal.
    Calvin Coolidge,	July 4 1872,	Plymouth Vermont,	Jan 5 1933,	Northampton Mass.
    Herbert Hoover,	Aug 10 1874,	West Branch Iowa,	Oct 20 1964,	New York New York
    Franklin Roosevelt,	Jan 30 1882,	Hyde Park New York,	Apr 12 1945,	Warm Springs Georgia
    Harry S. Truman,	May 8 1884,	Lamar Missouri,	Dec 26 1972,	Kansas City Missouri
    Dwight Eisenhower,	Oct 14 1890,	Denison Texas,	Mar 28 1969,	Washington D.C.
    John F. Kennedy,	May 29 1917,	Brookline Mass.,	Nov 22 1963,	Dallas Texas
    Lyndon B. Johnson,	Aug 27 1908,	Gillespie Co. Texas,	Jan 22 1973,	Gillespie Co. Texas
    Richard Nixon,	Jan 9 1913,	Yorba Linda Cal.,	Apr 22 1994,	New York New York
    Gerald Ford,	July 14 1913,	Omaha Nebraska,	Dec 26 2006,	Rancho Mirage Cal.
    Jimmy Carter,	Oct 1 1924,	Plains Georgia,	,
    Ronald Reagan,	Feb 6 1911,	Tampico Illinois,	June 5 2004,	Los Angeles Cal.
    George Bush,	June 12 1924,	Milton Mass.,	,
    Bill Clinton,	Aug 19 1946,	Hope Arkansas,	,
    George W. Bush,	July 6 1946,	New Haven Conn.,	,
    Barack Obama,	Aug 4 1961,	Honolulu Hawaii,	,"));
    }
}
