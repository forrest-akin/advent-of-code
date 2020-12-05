use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::num::ParseIntError;


fn main() {
    let raw_input = read_input( "input" ).expect( "unable to read input!" );
    let input = parse_input( &raw_input ).expect( "failed to parse input!" );
    let (x, y, z) = find_three_sum( 2020, &input ).expect( "no matching triple found for input!" );
    serialize_output( x, y, z, x * y * z )
}

fn read_input( file_path: &str ) -> Result<String, Error> {
    fs::read_to_string( file_path )
}

fn parse_input( input: &str ) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

fn find_three_sum( target: i32, numbers: &[i32] ) -> Option<(i32, i32, i32)> {
    let diff_map = key_by_diff( target, &numbers );
    numbers.iter().find_map( |&x|
        numbers.iter().find_map( |&y|
            diff_map.get( &(x + y) ).map( |&z| (x, y, z) ) ) )
}

fn key_by_diff( target: i32, numbers: &[i32] ) -> HashMap<i32, i32> {
    numbers.iter().map( |&x| ( target - x, x ) ).collect()
}

fn serialize_output( x: i32, y: i32, z: i32, result: i32 ) {
    println!( "{{ \"entries\": \"({}, {}, {})\", \"result\": {} }}"
            , x, y, z, result );
}
