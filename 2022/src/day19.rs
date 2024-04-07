use enum_map::{enum_map, EnumMap};

use strum::IntoEnumIterator;

use crate::day0::Day;

pub struct Day19<const MIN: u32>;

impl<const MIN: u32> Day<19, Vec<Blueprint>, Currency> for Day19<MIN> {
    fn solve(input: Vec<Blueprint>) -> Currency {
        input.into_iter().map( |b| b.id * Day19::<MIN>::solve_blueprint(b)).sum()
    }

    fn parse(input: &str) -> Vec<Blueprint> {
        input.lines().map(Blueprint::parse).collect()
    }
}

impl<const MIN: u32> Day19<MIN> {
    fn solve_blueprint(b: Blueprint) -> Currency {
   
        let mut queue = vec![Factory::initial()];

        for _minute in 1..=MIN {
            println!("== Minute {_minute} ==");
            println!("== Size of queue: {} ==", queue.len());
            let mut new_queue = vec![];
            for state in queue {
                for new_factory in Strategy::get_stupid_moves(&b, state){
                    dbg!(new_factory.clone());
                    if new_queue.iter().all(|factory| !(factory > &new_factory)){
                        new_queue.push(new_factory)
                    }
                }
            }
            queue = new_queue;
        }
        
        queue.into_iter().map(|factory| factory.states[Material::Geode].stock).max().unwrap()
    }
}

struct Strategy;

impl Strategy
{

    fn get_moves(b : &Blueprint, factory : &Factory) -> Vec<Factory> {
        let mut moves = vec![factory.clone()];

        for (material, price) in b.prices {
            if factory.can_buy(material, price) {
                let mut new_factory = factory.clone();
                new_factory.buy(material, price);
                moves.push(new_factory);
            } 
        }

        for factory in moves.iter_mut(){
            factory.work();
        }
        
        moves
    }

    fn get_stupid_moves(b : &Blueprint, mut factory : Factory) -> Vec<Factory> {

        let mut buy = false;
        for (material, &price) in b.prices.iter().rev() {
            if factory.can_buy(material, price) && factory.states[material].machines <= 4 {
                factory.work();
                factory.buy(material, price);
                buy = true;
                break;
            } 
        }
        if !buy
        {
            factory.work();
        }

        

        vec![factory]
    }        

    
    fn get_stupid_moves2(b : &Blueprint, mut factory : Factory) -> Vec<Factory> {

        for (material, &price) in b.prices.iter().rev() {
            if factory.can_buy(material, price) && factory.states[material].machines <= 4 {
                factory.buy(material, price);
                break;
            } 
        }

        factory.work();

        vec![factory]
    }        
}

// trait Strategy {
//     fn do_moves(states : Vec<EnumMap<Material, MaterialState>>) -> Vec<EnumMap<Material, MaterialState>>;
// }
#[derive(Debug,Clone, PartialEq)]
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

}

impl PartialOrd for Factory
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // dbg!(self);
        // dbg!(other);
       let result = Material::iter().map(|m| self.states[m].partial_cmp(&other.states[m])).reduce(|acc, next| if acc == next { acc } else {Option::None}) .flatten();
        // dbg!(result);
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.stock == other.stock && self.machines == other.machines {
            return Some(std::cmp::Ordering::Equal)
        }
        match (self.stock >= other.stock, self.machines >= other.machines) {
            (true, true) => Some(std::cmp::Ordering::Greater),
            (false, true) => None,
            (true, false) => None,
            (false, false) => Some(std::cmp::Ordering::Less),
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
