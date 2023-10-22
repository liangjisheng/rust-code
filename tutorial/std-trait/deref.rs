// Deref<Target = T>类型可以使用*操作符解引用为T类型。这在像Box和Rc这样的智能指针类型
// 中有很明显的用例。尽管如此，但是我们在 Rust 代码中很少见到这种显式的解引用操作，这是因为
// Rust 有一个被称为解引用强制转换（deref coercion）的特性。

// 当类型被作为函数参数传递、从函数返回或者作为方法调用的一部分时，Rust 会自动对这些类型进行
// 解引用。这也解释了为什么我们可以在一个期望&str和&[T]的函数中可以传入 &String和&Vec<T>
// 因为String实现了Deref<Target = str>并且Vec<T>实现了Deref<Target = [T]>

// Deref和DerefMut应该仅被实现于智能指针类型。人们误用和滥用这些 trait 的最常见的方式是
// 试图把 OOP（面向对象程序设计）风格的数据继承塞进 Rust 中。这样是行不通的。Rust 不是 OOP
// 让我们进行一些测试，来看看它是在哪里、怎么样以及为什么行不通。让我们从下面的例子开始

use std::ops::AddAssign;
use std::ops::Deref;
use std::ops::DerefMut;

struct Human {
    health_points: u32,
}

enum Weapon {
    Spear,
    Axe,
    Sword,
}

// a Soldier is just a Human with a Weapon
struct Soldier {
    human: Human,
    weapon: Weapon,
}

impl Deref for Soldier {
    type Target = Human;
    fn deref(&self) -> &Human {
        &self.human
    }
}

enum Mount {
    Horse,
    Donkey,
    Cow,
}

// a Knight is just a Soldier with a Mount
struct Knight {
    soldier: Soldier,
    mount: Mount,
}

impl Deref for Knight {
    type Target = Soldier;
    fn deref(&self) -> &Soldier {
        &self.soldier
    }
}

#[derive(Clone)]
enum Spell {
    MagicMissile,
    FireBolt,
    ThornWhip,
}

// a Mage is just a Human who can cast Spells
struct Mage {
    human: Human,
    spells: Vec<Spell>,
}

impl Deref for Mage {
    type Target = Human;
    fn deref(&self) -> &Human {
        &self.human
    }
}

enum Staff {
    Wooden,
    Metallic,
    Plastic,
}

// a Wizard is just a Mage with a Staff
struct Wizard {
    mage: Mage,
    staff: Staff,
}

impl Deref for Wizard {
    type Target = Mage;
    fn deref(&self) -> &Mage {
        &self.mage
    }
}

fn borrows_human(human: &Human) {}
fn borrows_soldier(soldier: &Soldier) {}
fn borrows_knight(knight: &Knight) {}
fn borrows_mage(mage: &Mage) {}
fn borrows_wizard(wizard: &Wizard) {}

fn example(human: Human, soldier: Soldier, knight: Knight, mage: Mage, wizard: Wizard) {
    // all types can be used as Humans
    borrows_human(&human);
    borrows_human(&soldier);
    borrows_human(&knight);
    borrows_human(&mage);
    borrows_human(&wizard);
    // Knights can be used as Soldiers
    borrows_soldier(&soldier);
    borrows_soldier(&knight);
    // Wizards can be used as Mages
    borrows_mage(&mage);
    borrows_mage(&wizard);
    // Knights & Wizards passed as themselves
    borrows_knight(&knight);
    borrows_wizard(&wizard);
}

// 首先，解引用强制转换仅作用于引用，因此，当我们想要传递所有权的时候它是行不通的
fn takes_human(human: Human) {}

fn example1(human: Human, soldier: Soldier, knight: Knight, mage: Mage, wizard: Wizard) {
    // all types CANNOT be used as Humans
    takes_human(human);
    // takes_human(soldier); // ❌
    // takes_human(knight); // ❌
    // takes_human(mage); // ❌
    // takes_human(wizard); // ❌
}

// 此外，解引用强制转换在泛型上下文中是无法工作的。假定我们仅在 humans 上实现某个 trait
trait Rest {
    fn rest(&self);
}

impl Rest for Human {
    fn rest(&self) {}
}

fn take_rest<T: Rest>(rester: &T) {
    rester.rest()
}

fn example2(human: Human, soldier: Soldier, knight: Knight, mage: Mage, wizard: Wizard) {
    // all types CANNOT be used as Rest types, only Human
    take_rest(&human);
    // take_rest(&soldier); // ❌
    // take_rest(&knight); // ❌
    // take_rest(&mage); // ❌
    // take_rest(&wizard); // ❌
}

// 而且，尽管解引用强制转换在很多场景都可以使用，但它不是万能的。它无法作用于操作数，
// 尽管操作符只是方法调用的语法糖。假定，我们想要Mage（魔术师）通过+=操作符学会Spell（拼写）

impl DerefMut for Wizard {
    fn deref_mut(&mut self) -> &mut Mage {
        &mut self.mage
    }
}

impl AddAssign<Spell> for Mage {
    fn add_assign(&mut self, spell: Spell) {
        self.spells.push(spell);
    }
}

fn example3(mut mage: Mage, mut wizard: Wizard, spell: Spell) {
    mage += spell.clone();
    // wizard += spell; // ❌ wizard not coerced to mage here
    wizard.add_assign(spell); // oof, we have to call it like this
}

fn main() {}
