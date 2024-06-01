fn main() {
    /*
    fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
        let mut largest = list[0];
        for element in list {
            if element > largest {
                largest = element;
            }
        }
        largest
    }

    let list = vec![1, 5, 10, 2, 4];

    let largest = get_largest(list);
    println!("The largest is {}", largest);
    */
    /*
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y
            }
        }
    }
    let p1 = Point { x: 10, y: 15 };
    let p2 = Point { x: 5, y: "Some string" };

    let p3 = p1.mixup(p2);

    println!("The x of p3 is {} and the y of p3 is {}", p3.x, p3.y);
    */
}
