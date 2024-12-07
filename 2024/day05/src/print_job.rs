use std::str::FromStr;

pub struct PrintJob {
    rule_set: OrderRuleSet,
    page_lists: Vec<PageList>
}

impl PrintJob {
    pub fn new(rule_set: OrderRuleSet, page_lists: Vec<PageList>) -> PrintJob {
        PrintJob { rule_set, page_lists }
    }

    pub fn calculate_score(&self) -> usize {
        let mut score = 0;

        for page in self.page_lists.iter() {
            if page.check_rule_set(&self.rule_set) {
                score += page.score();
            }
        }

        score
    }
}

pub struct OrderRuleSet {
    rules: Vec<OrderRule>
}

impl OrderRuleSet {
    pub fn new(rules: Vec<OrderRule>) -> OrderRuleSet {
        OrderRuleSet { rules }
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

impl FromStr for OrderRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("|");
        let left = split.next().and_then(|o| o.parse::<usize>().ok());
        let right = split.next().and_then(|o| o.parse::<usize>().ok());

        match (left, right) {
            (Some(left), Some(right)) => Ok(OrderRule::new(left, right)),
            _ => Err(())
        }
    }
}

pub struct PageList {
    page_numbers: Vec<usize>
}

impl PageList {
    pub fn new(page_numbers: Vec<usize>) -> PageList {
        PageList { page_numbers }
    }

    pub fn check_rule_set(&self, rule_set: &OrderRuleSet) -> bool {
        rule_set.rules.iter().all(|rule| self.check_rule(rule))
    }

    fn check_rule(&self, rule: &OrderRule) -> bool {
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

    pub fn score(&self) -> usize {
        let middle_index = self.page_numbers.len() / 2;

        self.page_numbers[middle_index]
    }
}

impl FromStr for PageList {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(",");
        let page_numbers: Result<Vec<usize>, _> = split.map(|e| e.parse::<usize>()).collect();

        match page_numbers {
            Ok(page_numbers) => Ok(PageList::new(page_numbers)),
            _ => Err(())
        }
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