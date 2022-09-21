#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 7, 3];

        let mut _v1_iter = v1.iter();  // K_22717 iter() Returns an iterator over the slice(切片引用)
        let mut v1_iter = v1.into_iter();  // K_22717 into_iter() Creates an iterator from a value
        let _b = &2;  // K_22717 _b 是一个指向数字2的地址?...
        assert_eq!(v1_iter.next(), Some(1));  // K_22717 第一次调用next(),返回第一个值 ， 返回 [`Some(Item)`] 
        assert_eq!(v1_iter.next(), Some(7));
        assert_eq!(v1_iter.next(), Some(3));
        // assert_eq!(v1_iter.next(), Some(&3));  原始的写法 v1.iter() 返回的是引用的迭代器 , ❓&7 是什么鬼?...指向一段内存地址,这个内存地址里存的是数字7,Some表示是Option类型
        assert_eq!(v1_iter.next(), None);  // K_22717 next()返回None,说明迭代器已经结束了
        // assert_eq!(v1.next(), None);  // K_22717 v1.next()返回None,说明迭代器已经结束了
    }
}
