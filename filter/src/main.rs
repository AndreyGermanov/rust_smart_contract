struct FilterCondition<T> {
    item: T,
}

impl<T: PartialOrd> FilterCondition<T> {
    fn is_match(&self,other_item:&T) -> bool {
        *other_item > self.item
    }
}

fn custom_filter<T:PartialOrd>(input:Vec<T>, cond: FilterCondition<T>) -> Vec<T> {
    input.into_iter().filter(|item| cond.is_match(item)).collect()
}

fn main() {
    let input = vec![1,2,3,4,5,6,7,8,9,10];
    let result = custom_filter(input, FilterCondition{item:5});
    println!("{:?}",result);
}
