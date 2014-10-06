use lolirofle::vector::Vector2 as Vector2;

enum Events{
    Move(Vector2<f32>),
    Jump(f32)
}

pub trait EventHandler{
	fn event(&mut self,event: Events);
}
