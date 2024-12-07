pub struct PrintJob {
    order_rules: Vec<OrderRule>,
    page_lists: Vec<PageList>
}

impl PrintJob {
    pub fn new(order_rules: Vec<OrderRule>, page_lists: Vec<PageList>) -> PrintJob {
        PrintJob { order_rules, page_lists }
    }

    pub fn calculate_score(&self) {

    }
}

pub struct OrderRule {
    left: usize,
    right: usize
}

impl OrderRule {
    pub fn new(left: usize, right: usize) -> OrderRule {
        OrderRule { left, right }
    }
}

pub struct PageList {
    page_numbers: Vec<usize>
}

impl PageList {
    pub fn new(page_numbers: Vec<usize>) -> PageList {
        PageList { page_numbers }
    }

    pub fn check_rule(&self, rule: &OrderRule) -> bool {
        let left_position = self.get_page_position(rule.left);
        let right_position = self.get_page_position(rule.right);

        match (left_position, right_position) {
            (Some(left_position), Some(right_position)) => left_position < right_position,
            _ => true
        }
    }

    fn get_page_position(&self, page_number: usize) -> Option<usize> {
        self.page_numbers.iter().position(|&page| page == page_number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    pub fn check_rule_passes() {
        let page_list = PageList::new(vec![75, 47, 61, 53, 29]);
        let rule = OrderRule::new(47, 53);

        assert_eq!(page_list.check_rule(&rule), true);
    }

    #[test]
    pub fn check_rule_fails() {
        let page_list = PageList::new(vec![75, 47, 61, 53, 29]);
        let rule = OrderRule::new(61, 75);

        assert_eq!(page_list.check_rule(&rule), false);
    }

    #[test]
    pub fn check_rule_left_missing_passes() {
        let page_list = PageList::new(vec![75, 47, 61, 53, 29]);
        let rule = OrderRule::new(47, 99);

        assert_eq!(page_list.check_rule(&rule), true);
    }

    #[test]
    pub fn check_rule_right_missing_passes() {
        let page_list = PageList::new(vec![75, 47, 61, 53, 29]);
        let rule = OrderRule::new(99, 53);

        assert_eq!(page_list.check_rule(&rule), true);
    }
}