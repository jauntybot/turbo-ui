turbo::init! {
    struct GameState {
        button: UIButton, // button
        toggle: bool, // toggle
    } = {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        Self {
            button: UIButton::new("Continue",(90, 44, 75, 30)), // create button from struct
            toggle: false // default toggle to false
        }
    }
}

// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec
turbo::go! ({
    // Hotload GameState
    let mut state = GameState::load();

    // Player input
    let m = mouse(0); // Mouse input
    let [mx, my] = m.position; // Mouse position
    
    // Check if mouse is hovering over button
    if let Some(b) = state.button.hover(state.button.hitbox, mx, my) {
        // Check if mouse clicked on button
        if m.left.just_pressed() {
            b.click(); // Call function local to button
            state.toggle = !state.toggle; // Flip toggle
        }
    }

    // Effect of our toggle
    if state.toggle {
        text!("Goodnight, world!!!");
    } else {
        text!("Hello, world!!!");
    }
    
    // Draw
    state.button.draw();

    // Save GameState
    state.save();
});


#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct UIButton {
    pub hitbox: (i32, i32, i32, i32),
    pub text: String,
    pub hovered: bool,
    pub count: u32,
}

impl UIButton {
    pub fn new (text: &str, hitbox: (i32, i32, i32, i32)) -> Self {
        Self {
            hitbox, // x, y, w, h
            text: text.to_string(), // button text
            hovered: false, // hover state
            count: 0, // count of clicks
        }
    }

    pub fn draw(&self) {
        // Color references
        let (c1, c2): (u32, u32) = match self.hovered {
            true => (0x323b42ff, 0xffffffff),
            false => (0xffffffff, 0x323b42ff)
        };
        // Calculate text offset for centering
        let (x, y) = 
            (self.hitbox.0 + (self.hitbox.2/2) - (self.text.len() as f32 * 2.5) as i32, 
            self.hitbox.1 + (self.hitbox.3/2) - 3);

        // Draw button
        rect!(x = self.hitbox.0, y = self.hitbox.1, w = self.hitbox.2, h = self.hitbox.3, color = c1);
        // Draw text
        text!(&self.text, x = x, y = y, color = c2);
        text!("{:?}", self.count; x = x, y = y + 8, color = c2);
    }
}

// Universal trait to be implemented by any struct that is clickable
pub trait Clickable {
    // Function that checks if given mouse position is hovering over hitbox,
    // Returns Option(&mut Self), which is Type ambiguous. This allows any struct that implements this trait to return a mutable reference to itself or None
    fn hover(&mut self, hitbox: (i32, i32, i32, i32), mx: i32, my: i32) -> Option<&mut Self> {
        if mx >= hitbox.0 && mx <= hitbox.0 + hitbox.2
        && my >= hitbox.1 && my <= hitbox.1 + hitbox.3 {
            Clickable::hover_state(self, true);
            return Some(self)
        } else {
            Clickable::hover_state(self, false);
            return None
        }
    }

    // Private function for toggling hover state
    fn hover_state(&mut self, hover: bool) {}

    // Private function for registering clicks on button
    fn click(&mut self) {}
}

// Implement Clickable for UIButton and override private functionality
impl Clickable for UIButton {
    fn hover_state(&mut self, hover: bool) {
        self.hovered = hover; // Toggle hover state
    }

    fn click(&mut self) {
        self.count += 1; // Count click
    }
}