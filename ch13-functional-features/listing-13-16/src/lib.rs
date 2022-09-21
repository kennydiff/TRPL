#[cfg(test)]
mod tests {
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();
        
        /*
        for item in v1_iter {  // K_22717 v1_iter经过sum()这种方法调用之后，所有权被移动了，所以这里不能再使用v1_iter了
            println!("{}", item);
        } */

        assert_eq!(total, 6);
    }
}
