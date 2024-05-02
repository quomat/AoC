use std::{cmp::Ordering, collections::HashSet};

use enum_map::{enum_map, EnumMap};

use strum::IntoEnumIterator;

use crate::day0::Day;

pub struct Day19<const MIN: u32>;

impl<const MIN: u32> Day<19, Vec<Blueprint>, Currency> for Day19<MIN> {
    fn solve(input: Vec<Blueprint>) -> Currency {
        input.into_iter().map( |b| b.id * Day19::<MIN>::solve_blueprint(b)).sum()
    }

    fn solve2(input: Vec<Blueprint>) -> Currency {
        input.into_iter().take(3).map( |b| Day19::<MIN>::solve_blueprint(b)).product()
    }

    fn parse(input: &str) -> Vec<Blueprint> {
        input.lines().map(Blueprint::parse).collect()
    }
}

impl<const MIN: u32> Day19<MIN> {
    fn solve_blueprint(b: Blueprint) -> Currency {
   
        let mut queue = HashSet::new();
        queue.insert(Factory::initial());
        for minute in 1..=MIN-1 {
            let mut new_queue = HashSet::new();
            let mut max_geodes = 0;
            println!("== Minute {minute} ==");
            println!("== Size of queue: {} ==", queue.len());
            for mut state in queue {
                let moves = Self::get_moves(&b, &state);
                state.work();
                for mov in moves{
                    
                    let mut new_factory = state.clone();
                    mov.inspect(|&m| new_factory.buy(m, b.prices[m]));
                    let (new_max, new_potential_max) = Self::max_geodes(minute, &new_factory, &b); 
                    if  new_potential_max > max_geodes {
                        if new_max > max_geodes {
                            new_queue.clear();
                            max_geodes = new_max;
                        }
                        new_queue.insert(new_factory);
                    }                    
                }
            }
            println!("  = max_geodes = {}",max_geodes);
            queue = new_queue;
        }
        
        queue.into_iter().map(|mut factory| { factory.work(); factory.states[Material::Geode].stock}).max().unwrap()
    }


    
    fn get_moves(b : &Blueprint, factory : &Factory) -> Vec<Option<Material>> {
        let mut moves = vec![None];

        for (material, price) in b.prices {
            if factory.can_buy(material, price) && BarbarianForce::strategize(b, factory, material, price) {
                moves.push(Some(material))
                
            } 
        }
    
        moves
    }


    fn max_geodes(m : u32, factory : &Factory, blueprint : &Blueprint) -> (u32,u32)
    {
        let days_left = MIN - m;
        let maximum_output = days_left * (days_left + 1) / 2;
        let geodes = factory.get_guaranteed(Material::Geode, days_left);

        let ores = factory.get_guaranteed(Material::Ore, days_left);
        let clays = factory.get_guaranteed(Material::Clay, days_left);
        let obsidians = factory.get_guaranteed(Material::Obsidian, days_left);
        
        let geode_price = blueprint.prices[Material::Geode];
        let geode_factories = core::cmp::min(ores / geode_price.ore_price, obsidians / geode_price.previous_price.unwrap());
        let geode_factories = core::cmp::min(geode_factories, days_left);

        (geodes,geodes + maximum_output)
    }

    fn is_optimal(new_queue: &mut Vec<Factory>, new_factory: &Factory) -> bool {
        
        let mut flag : bool = true;
        let mut i = 0;
        while i < new_queue.len()
        {
            match new_queue[i].partial_cmp(new_factory)
            {
                Some(Ordering::Greater) => {
                    flag = false;
                    break;
                }
                Some(Ordering::Less) => {
                    new_queue.swap_remove(i);
                }
                Some(Ordering::Equal) | None => i += 1,
            }
        }
        flag 
    }
}

struct Bruteforce;

trait Strategy
{
    fn strategize(b: &Blueprint, factory: &Factory, material: Material, price: Price) -> bool;
}

impl Strategy for Bruteforce{
    fn strategize(b: &Blueprint, factory: &Factory, material: Material, price: Price) -> bool {
        true
    }
}

struct BarbarianForce;

impl Strategy for BarbarianForce{
    
    fn strategize(b: &Blueprint, factory: &Factory, material: Material, price: Price) -> bool {
        let ore_max = [b.prices[Material::Ore].ore_price, b.prices[Material::Clay].ore_price, b.prices[Material::Obsidian].ore_price, b.prices[Material::Geode].ore_price].into_iter().max().unwrap();
        let clay_max = b.prices[Material::Obsidian].previous_price.unwrap();
        let obsidian_max = b.prices[Material::Geode].previous_price.unwrap();
        let material_max = enum_map!{
            Material::Ore => ore_max,
            Material::Clay => clay_max,
            Material::Obsidian => obsidian_max,
            Material::Geode => Currency::max_value(),
           
        };
        
        factory.states[material].machines < material_max[material]
    }
}

// trait Strategy {
//     fn do_moves(states : Vec<EnumMap<Material, MaterialState>>) -> Vec<EnumMap<Material, MaterialState>>;
// }
#[derive(Debug,Clone, PartialEq, Eq, Hash)]
struct Factory 
{
  states : EnumMap<Material, MaterialState>
}

impl Factory
{
    fn initial() -> Factory{
        
        Factory { states: enum_map! { Material::Ore => MaterialState::new1(),  _ => MaterialState::new()} }
    }

    fn can_buy(&self, material : Material, price : Price) -> bool
    {
        self.states[Material::Ore].stock >= price.ore_price && (price.previous_price.is_none() || price.previous_price.is_some_and(|p| self.states[material.previous()].stock >= p))
    }

    fn buy(& mut self, material : Material, price : Price)
    {
        self.states[Material::Ore].stock -= price.ore_price;
        price.previous_price.inspect(|p| self.states[material.previous()].stock -= p);
        
        self.states[material].machines += 1;
    }

    fn work(&mut self)
    {
        for material in Material::iter()
        {
            self.states[material].stock += self.states[material].machines
        }    
    }

    fn get_guaranteed(&self, material : Material, days : u32) -> Currency
    {
        let state = self.states[material];

        state.machines * days + state.stock
    }

}

impl PartialOrd for Factory
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // dbg!(self);
        // dbg!(other);
       let result = Material::iter().map(|m| self.states[m].partial_cmp(&other.states[m])).reduce(|acc, next| if acc == next { acc } else {Option::None}) .flatten();
        // dbg!(result);
        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MaterialState {
    stock: Currency,
    machines: Currency,
}

impl MaterialState {
    fn new() -> MaterialState {
        MaterialState {
            stock: 0,
            machines: 0,
        }
    }

    fn new1() -> MaterialState
    {
        MaterialState {
            stock : 0,
            machines: 1
        }
    }

}

impl PartialOrd for MaterialState
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.stock == other.stock && self.machines == other.machines {
            return Some(Ordering::Equal)
        }
        match (self.stock >= other.stock, self.machines >= other.machines) {
            (true, true) => Some(Ordering::Greater),
            (false, true) => None,
            (true, false) => None,
            (false, false) => Some(Ordering::Less),
        }
    }
}

#[derive(strum_macros::EnumIter, enum_map::Enum, Debug, Clone, Copy)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Material {
    fn previous(self) -> Material
    {
        match self {
            Material::Ore => Material::Ore,
            Material::Clay => Material::Ore,
            Material::Obsidian => Material::Clay,
            Material::Geode => Material::Obsidian,
        }
    }
}

type Currency = u32;

#[derive(Debug, Clone, Copy)]
struct Price {
    ore_price: Currency,
    previous_price: Option<Currency>,
}

impl Price {
    fn only_ore(c: Currency) -> Price {
        Price {
            ore_price: c,
            previous_price: None,
        }
    }

    fn new(ore: Currency, previous: Currency) -> Price {
        Price {
            ore_price: ore,
            previous_price: Some(previous),
        }
    }
}

#[derive(Debug)]
pub struct Blueprint {
    id : u32,
    prices: EnumMap<Material, Price>,
}

impl Blueprint {
    fn parse(s: &str) -> Blueprint {
        let (ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian);
        let n: Currency;
        text_io::scan!(s.bytes() => "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.",

        n, ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian);

        Blueprint {
            id : n,
            prices: enum_map! { Material::Ore => Price::only_ore(ore_ore),
            Material::Clay => Price::only_ore(clay_ore),
            Material::Obsidian => Price::new(obsidian_ore,obsidian_clay),
            Material::Geode => Price::new(geode_ore,geode_obsidian)
            },
        }
    }
}
