use tui::{widgets::ListItem, text::{Span, Spans, Text}, style::{Style, Color}};
use std::{ops::{Index, IndexMut}, mem::discriminant};
use crate::items::{ItemKind, axe::Axe, pickaxe::Pickaxe, stone::Stone, stick::Stick, iron::Iron, wood::Wood};

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum Recipe {
    Axe,
    Pickaxe,
    Boat,
    Armor,
    Sword,
    Bow,
    Arrow
}

impl Recipe {
    pub fn get_item(&self) -> ItemKind {
        match self {
            Recipe::Axe => ItemKind::Axe(Axe::new(1)),
            Recipe::Pickaxe => ItemKind::Pickaxe(Pickaxe::new(1)),
            Recipe::Boat => ItemKind::Axe(Axe::new(1)),
            Recipe::Armor => ItemKind::Axe(Axe::new(1)),
            Recipe::Arrow => ItemKind::Axe(Axe::new(1)),
            Recipe::Sword => ItemKind::Axe(Axe::new(1)),
            Recipe::Bow => ItemKind::Axe(Axe::new(1)),
        }
    }

    pub fn name<'a>(&self) -> String {
        let s = match self {
            Recipe::Pickaxe => "pickaxe",
            Recipe::Axe => "axe",
            Recipe::Arrow => "arrow",
            Recipe::Armor => "armor",
            Recipe::Bow => "bow",
            Recipe::Boat => "boat",
            Recipe::Sword => "sword"
        };
        String::from(s)
    }

    pub fn needs(&self) -> Vec<(ItemKind, i8)> {
        match self {
            Recipe::Pickaxe => vec![(ItemKind::Stone(Stone::new(1)), 5), (ItemKind::Stick(Stick::new(1)), 5)],
            Recipe::Axe => vec![(ItemKind::Stone(Stone::new(1)), 5), (ItemKind::Stick(Stick::new(1)), 5)],
            Recipe::Arrow => vec![(ItemKind::Stone(Stone::new(1)), 1), (ItemKind::Stick(Stick::new(1)), 1)],
            Recipe::Armor => vec![(ItemKind::Iron(Iron::new(1)), 20)],
            Recipe::Boat => vec![(ItemKind::Wood(Wood::new(1)), 20)],
            Recipe::Bow => vec![(ItemKind::Stick(Stick::new(1)), 20)],
            Recipe::Sword => vec![(ItemKind::Iron(Iron::new(1)), 10)],
        }
    }

    pub fn information(&self, inventory: &Inventory) -> Text {
        let mut text = Vec::new();
        for (item, amount) in self.needs() {
            let total_quantity = inventory.total_quantity(&item);
            let color = if total_quantity < amount as u32 {
                Color::Red
            } else { Color::White };
            let s1 = format!("{}: ", item.name());
            let s2 = format!("{total_quantity}/{amount}");
            let spans = vec![
                Span::raw(s1),
                Span::styled(s2, Style::default().fg(color))
            ];
            text.push(Spans::from(spans));
        }
        Text::from(text)
    }

    pub fn item_list(inventory: &Inventory) -> Vec<ListItem> {
        Recipe::recipes()
            .iter()
            .map(|x| {
                let color = if inventory.can_craft(x) {Color::White} else {Color::DarkGray};
                ListItem::new(Span::styled(x.name(), Style::default().fg(color)))
            })
            .collect()
    }

    pub fn recipes() -> Vec<Recipe> {
        vec![
            Recipe::Pickaxe,
            Recipe::Axe,
            Recipe::Armor,
            Recipe::Arrow,
            Recipe::Boat,
            Recipe::Sword,
        ]
    }
}

pub struct Inventory(Vec<ItemKind>);

impl Inventory {
    /// Create new empty inventory
    pub fn new() -> Self {
        Inventory(Vec::new())
    }

    /// add an item to the inventory
    pub fn add(&mut self, mut item_to_add: ItemKind) {
        for item in &mut self.0 {
            if discriminant(item) == discriminant(&mut item_to_add) {
                item.change_quantity(item_to_add.quantity());
                return;
            }
        }
        self.0.push(item_to_add);
    }

    pub fn total_quantity(&self, item_type: &ItemKind) -> u32 {
        let mut total: u32 = 0;
        for item in &self.0 {
            if discriminant(item) == discriminant(&item_type) {
                total += item.quantity() as u32;
            }
        }
        total
    }

    pub fn can_craft(&self, recipe: &Recipe) -> bool {
        for (item, amount) in recipe.needs() {
            if self.total_quantity(&item) < amount as u32 {
                return false;
            }
        }
        true
    }

    pub fn craft(&mut self, recipe: &Recipe) -> String {
        if self.can_craft(&recipe) {
            for (item_needed, mut amount) in recipe.needs() {
                for item in &mut self.0 {
                    if discriminant(item) == discriminant(&item_needed) {
                        amount = item.change_quantity(-amount);
                    }
                }
            }
            self.add(recipe.get_item());
            String::from("crafting")
        } else {
            String::from("not enough")
        }
    }

    pub fn get(&mut self, index: usize) -> &mut ItemKind {
        &mut self.0[index]
    }

    pub fn to_item_list(&self) -> Vec<ListItem> {
        let mut listitem = Vec::new();
        for item in &self.0 {
            listitem.push(ListItem::new(item.shape()));
        }
        listitem
    }

    pub fn to_extended_item_list(&self) -> Vec<ListItem> {
        let mut listitem = Vec::new();
        for item in &self.0 {
            let spans = Spans::from(vec![
                Span::from(item.name()),
                Span::from(" ["),
                item.shape(),
                Span::from("] x"),
                Span::from(item.quantity().to_string()),
            ]);
            listitem.push(ListItem::new(spans));
        }
        listitem
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<usize> for Inventory {
    type Output = ItemKind;

    fn index(&self, i: usize) -> &ItemKind {
        &self.0[i]
    }
}

impl IndexMut<usize> for Inventory {
    fn index_mut(&mut self, i: usize) -> &mut ItemKind {
        &mut self.0[i]
    }
}
