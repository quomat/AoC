use enum_map::{EnumMap, enum_map};

use crate::day0::Day;

pub struct Day19<const MIN : u32>;

impl<const MIN : u32> Day<2022,19,Vec<Blueprint>,Currency> for Day19<MIN>
{
    fn solve(input: Vec<Blueprint>) -> Currency {

        todo!()
    }

    fn parse(input: &str) -> Vec<Blueprint> {
        input.lines().map(Blueprint::parse).collect()
    }
}

impl<const MIN :  u32> Day19<MIN>
{
    fn solve_blueprint(b : Blueprint) -> Currency
    {
        let mut initial_state = enum_map! { Material::Ore | _ => StateElement::new()} ;

        initial_state[Material::Ore].machines = 1;
        
        let mut queue = vec![initial_state];

        todo!()
    }
}

struct StateElement
{
    stock : Currency,
    machines : Currency,
}

impl StateElement
{
    fn new() -> StateElement
    {
        StateElement { stock : 0, machines : 0}
    }
}

#[derive(enum_map::Enum, Debug)]
enum Material
{
    Ore,
    Clay,
    Obsidian,
    Geode
}

type Currency = u32;

#[derive(Debug)]
struct Price
{
    ore_price : Currency,
    previous_price : Option<Currency>
}

impl Price
{
    fn only_ore(c : Currency) -> Price
    {
        Price { ore_price : c, previous_price : None}
    }

    fn new(ore : Currency, previous : Currency) -> Price
    {
        Price {ore_price : ore, previous_price: Some(previous)}
    }
}

#[derive(Debug)]
pub struct Blueprint
{
    prices : EnumMap<Material,Price>
}

impl Blueprint
{
    fn parse(s : &str) -> Blueprint
    {
        let (ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian);
        let n : Currency;
        text_io::scan!(s.bytes() => "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.",

        n, ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian); 

        Blueprint {
            prices : enum_map! { Material::Ore => Price::only_ore(ore_ore),
                Material::Clay => Price::only_ore(clay_ore),
                Material::Obsidian => Price::new(obsidian_ore,obsidian_clay),
                Material::Geode => Price::new(geode_ore,geode_obsidian)
                }
            }

    }
}
