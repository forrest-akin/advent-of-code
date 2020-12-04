use std::collections::HashMap;
use std::fs;


fn main() {
    match find_three_sum( 2020, &parse_input( &read_input( "input" ) ) ) {
        Some( ( x, y, z ) ) => serialize_output( x, y, z, x * y * z ),
        _ => panic!( "No matching triple found for input!" ),
    }
}

fn read_input( file_path: &str ) -> String {
    fs::read_to_string( file_path ).expect( "unable to read input!" )
}

fn parse_input( input: &str ) -> Vec<i32> {
    match input.lines().map(str::parse).collect() {
        Ok( input ) => input,
        Err( error ) => panic!( "failed to parse input: {}", error ),
    }
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
