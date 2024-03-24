use lazy_static::lazy_static;
use regex::Regex;

pub fn parse_tokens(bqsql: &str) -> Vec<[usize; 3]> {
    lazy_static! {
        //`.*`|
        static ref RE: Regex = Regex::new(r"\d*\.{1}\d*|[A-Za-z0-9_]+|\W?").unwrap();
    }

    let mut tokens: Vec<[usize; 3]> = Vec::new();

    let mut line_index: usize = 0;
    for line in bqsql.lines() {
        let mut gaps: Vec<[usize; 2]> = Vec::new();
        let mut previous_gap_start: usize = 0;

        for p1 in find_strings_and_line_comments(line) {
            gaps.push([previous_gap_start, p1[0]]);
            previous_gap_start = p1[1];
            tokens.push([line_index, p1[0], p1[1]]);
        }

        if gaps.len() == 0 {
            gaps.push([0, line.len()])
        } else {
            if previous_gap_start < line.len() {
                gaps.push([previous_gap_start, line.len()]);
            }
        }

        for gap in gaps {
            if gap[0] == gap[1] {
                continue;
            }

            let adjusted_line = &line[gap[0]..gap[1]];

            for m in RE.find_iter(adjusted_line) {
                let partial = &adjusted_line[m.start()..m.end()];

                // println!("{}", partial.to_string());

                if partial.trim().len() > 0 {
                    tokens.push([line_index, gap[0] + m.start(), gap[0] + m.end()]);
                }
            }
        }

        line_index = line_index + 1;
    }

    tokens.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));

    tokens
}

#[cfg(test)]
mod tests_parse_tokens {
    use crate::bqsql_document::token_parser::parse_tokens;

    #[test]
    fn parse_tokens_single_line_operation() {
        let result =
            parse_tokens("    SELECT 23+2.45 --test, another `table` 123 \"back\" to 'dust'");

        assert_eq!(5, result.len());
        assert_eq!([0, 4, 10], result[0]);
        assert_eq!([0, 11, 13], result[1]);
        assert_eq!([0, 13, 14], result[2]);
        assert_eq!([0, 14, 18], result[3]);
        assert_eq!([0, 19, 63], result[4]);
    }

    #[test]
    fn parse_tokens_single_line_string() {
        let result = parse_tokens(
            "SELECT \"this is a ''' string \" --test, another `table` 123 \"back\" to 'dust'",
        );

        assert_eq!(3, result.len());

        assert_eq!([0, 0, 6], result[0]);
        assert_eq!([0, 7, 30], result[1]);
        assert_eq!([0, 31, 75], result[2]);
    }

    #[test]
    fn parse_tokens_parenthisis() {
        let result = parse_tokens("SELECT (((1)))");

        assert_eq!(8, result.len());

        assert_eq!([0, 0, 6], result[0]);
        assert_eq!([0, 7, 8], result[1]);
        assert_eq!([0, 8, 9], result[2]);
        assert_eq!([0, 9, 10], result[3]);
        assert_eq!([0, 10, 11], result[4]);
        assert_eq!([0, 11, 12], result[5]);
        assert_eq!([0, 12, 13], result[6]);
        assert_eq!([0, 13, 14], result[7]);
    }

    #[test]
    fn parse_tokens_single_line_string_with_double_dash() {
        let result = parse_tokens("SELECT \"this is a -- string \"  ");

        assert_eq!(2, result.len());

        assert_eq!([0, 0, 6], result[0]);
        assert_eq!([0, 7, 29], result[1]);
    }

    #[test]
    fn parse_tokens_strings() {
        let result =
            parse_tokens(" SELECT 'this is a \\' -- string ',\"this is also a \\\" -- string \"");

        assert_eq!(4, result.len());

        assert_eq!([0, 1, 7], result[0]);
        assert_eq!([0, 8, 33], result[1]);
        assert_eq!([0, 33, 34], result[2]);
        assert_eq!([0, 34, 64], result[3]);
    }

    #[test]
    fn parse_tokens_strings_with_space() {
        let result =
            parse_tokens(" SELECT 'this is a \\' -- string ', \"this is also a \\\" -- string \"");

        assert_eq!(4, result.len());

        assert_eq!([0, 1, 7], result[0]);
        assert_eq!([0, 8, 33], result[1]);
        assert_eq!([0, 33, 34], result[2]);
        assert_eq!([0, 35, 65], result[3]);
    }

    #[test]
    fn parse_tokens_strings_multi_select() {
        let result =
            parse_tokens(" SELECT (SELECT AS STRUCT 2+2 AS asas, 'ASDASD' AS qweqwe) AS XXX");

        assert_eq!(17, result.len());

        assert_eq!([0, 1, 7], result[0]);
    }

    #[test]
    fn parse_tokens_single_quote_in_string() {
        let result = parse_tokens("SeLeCT 'Timmy O\\\'Hara'");

        assert_eq!(2, result.len());

        assert_eq!([0, 0, 6], result[0]);
        assert_eq!([0, 7, 22], result[1]);
    }

    #[test]
    fn parse_tokens_double_quote_in_string() {
        let result = parse_tokens("SELECT \"Timmy O'Hara\"");

        assert_eq!(2, result.len());

        assert_eq!([0, 0, 6], result[0]);
        assert_eq!([0, 7, 21], result[1]);
    }

    #[test]
    fn parse_tokens_single_quote_in_string_double_escape() {
        let result = parse_tokens("SELECT 'Timmy O\\\'Hara'");

        assert_eq!(2, result.len());

        assert_eq!([0, 0, 6], result[0]);
        assert_eq!([0, 7, 22], result[1]);
    }

    #[test]
    fn parse_tokens_single_quote_in_string_double_escape_multiple_columns() {
        let result = parse_tokens("SELECT 'Timmy O\\\'Hara', 2 AS second_column");

        assert_eq!(6, result.len());

        assert_eq!([0, 0, 6], result[0]);
        assert_eq!([0, 7, 22], result[1]);
        assert_eq!([0, 22, 23], result[2]);
        assert_eq!([0, 24, 25], result[3]);
        assert_eq!([0, 26, 28], result[4]);
        assert_eq!([0, 29, 42], result[5]);
    }

    #[test]
    fn parse_tokens_queries_file() {
        let bqsql = include_str!("query_files/queries.bqsql");

        let now = std::time::Instant::now();
        let result = parse_tokens(bqsql);
        let elapsed_time = now.elapsed();
        println!("took {} ms", elapsed_time.as_millis());

        assert!(result.len() > 30);

        let l0: Vec<[usize; 3]> = result.iter().filter(|l| l[0] == 0).map(|l| *l).collect();
        assert_eq!(1, l0.len());
        assert_eq!([0, 0, 6], l0[0]);

        let l1: Vec<[usize; 3]> = result.iter().filter(|l| l[0] == 1).map(|l| *l).collect();
        assert_eq!(4, l1.len());

        let l2: Vec<[usize; 3]> = result.iter().filter(|l| l[0] == 2).map(|l| *l).collect();
        assert_eq!(2, l2.len());

        let l3: Vec<[usize; 3]> = result.iter().filter(|l| l[0] == 3).map(|l| *l).collect();
        assert_eq!(2, l3.len());

        let l5: Vec<[usize; 3]> = result.iter().filter(|l| l[0] == 5).map(|l| *l).collect();
        assert_eq!(9, l5.len());

        let l7: Vec<[usize; 3]> = result.iter().filter(|l| l[0] == 7).map(|l| *l).collect();
        assert_eq!(9, l7.len());

        let l9: Vec<[usize; 3]> = result.iter().filter(|l| l[0] == 9).map(|l| *l).collect();
        assert_eq!(9, l9.len());
    }

    #[test]
    fn parse_tokens_square_brackets() {
        let result = parse_tokens("SELECT [1,2,3],4,5+1 FROM t");

        assert_eq!(16, result.len());

        assert_eq!([0, 0, 6], result[0]);
        assert_eq!([0, 7, 8], result[1]);
        assert_eq!([0, 8, 9], result[2]);
        assert_eq!([0, 9, 10], result[3]);
        assert_eq!([0, 10, 11], result[4]);
        assert_eq!([0, 11, 12], result[5]);
        assert_eq!([0, 12, 13], result[6]);
        assert_eq!([0, 13, 14], result[7]);
        assert_eq!([0, 14, 15], result[8]);
        assert_eq!([0, 15, 16], result[9]);
        assert_eq!([0, 16, 17], result[10]);
        assert_eq!([0, 17, 18], result[11]);
        assert_eq!([0, 18, 19], result[12]);
        assert_eq!([0, 19, 20], result[13]);
        assert_eq!([0, 21, 25], result[14]);
        assert_eq!([0, 26, 27], result[15]);
    }

    #[test]
    fn from_statement_with_full_table_name() {
        let result = parse_tokens(
            r#"
        SELECT 
            pimExportDate, 
            Combi_number,
            columnC,
            
            -- Flavour_Copy
        FROM `damiao-project-1.PvhTest.PimExport` pim
        WHERE 
            pimExportDate = "2022-03-23"
            -- AND (
            --     Combi_number = '0000F3223E001'
            --     OR Combi_number = "0000F2934E101"
            -- )
        LIMIT 101;"#,
        );

        assert_eq!(22, result.len());

        assert_eq!([7, 13, 49], result[9]);
    }
}

fn find_strings_and_line_comments(line: &str) -> Vec<[usize; 2]> {
    let mut possible_escape_char = false;
    let mut possible_line_comment = false;

    let mut positions: Vec<[usize; 2]> = Vec::new();
    let mut index: usize = 0;
    let mut previous_double_quote: Option<usize> = None;
    let mut previous_single_quote: Option<usize> = None;
    let mut previous_accent: Option<usize> = None;

    for character in line.chars() {
        if character == '\\' {
            possible_escape_char = true;
            index = index + 1;
            continue;
        }
        if character == '#' && previous_double_quote.is_none() && previous_single_quote.is_none() {
            positions.push([index, line.len()]);
            return positions;
        }

        if character == '-' && previous_accent.is_none()  && previous_double_quote.is_none() && previous_single_quote.is_none() {
            if possible_line_comment {
                positions.push([index - 1, line.len()]);
                return positions;
            }
            possible_line_comment = true;
            index = index + 1;
            continue;
        }

        if character == '"' && previous_single_quote.is_none() {
            if !possible_escape_char {
                if previous_double_quote.is_some() {
                    positions.push([previous_double_quote.unwrap(), index + 1]);
                    previous_double_quote = None;
                } else {
                    previous_double_quote = Some(index);
                }
            }
        }

        if character == '`'
            && previous_double_quote.is_none()
            && previous_single_quote.is_none()
        {
            if !possible_escape_char {
                if previous_accent.is_some() {
                    positions.push([previous_accent.unwrap(), index + 1]);
                    previous_accent = None;
                } else {
                    previous_accent = Some(index);
                }
            }
        }

        if character == '\'' && previous_double_quote.is_none() {
            if !possible_escape_char {
                if previous_single_quote.is_some() {
                    positions.push([previous_single_quote.unwrap(), index + 1]);
                    previous_single_quote = None;
                } else {
                    previous_single_quote = Some(index);
                }
            }
        }

        possible_escape_char = false;
        index = index + 1;
    }

    positions
}

#[cfg(test)]
mod tests_find_strings_and_line_comments {
    use crate::bqsql_document::token_parser::find_strings_and_line_comments;

    #[test]
    fn find_strings_and_line_comments_no_string_no_comment() {
        let result = find_strings_and_line_comments(" SELECT 23-2.45");

        assert_eq!(0, result.len());
    }

    #[test]
    fn find_strings_and_line_comments_no_string_with_comment() {
        let result = find_strings_and_line_comments(
            " SELECT 23+2.45 --test, another `table` 123 \"back\" to 'dust'",
        );

        assert_eq!(1, result.len());
        assert_eq!([16, 60], result[0]);
    }

    #[test]
    fn find_strings_and_line_comments_double_quote_string_and_comment() {
        let result = find_strings_and_line_comments(
            " SELECT \"this is a \\\" -- string \" --test, another `table` 123 \"back\" to 'dust'",
        );

        assert_eq!(2, result.len());
        assert_eq!([8, 33], result[0]);
        assert_eq!([34, 78], result[1]);
    }

    #[test]
    fn find_strings_and_line_comments_single_quote_string_and_comment() {
        let result = find_strings_and_line_comments(
            " SELECT 'this is a \\' -- string ' --test, another `table` 123 \"back\" to 'dust'",
        );

        assert_eq!(2, result.len());
        assert_eq!([8, 33], result[0]);
        assert_eq!([34, 78], result[1]);
    }

    #[test]
    fn find_strings_and_line_comments_strings() {
        let result = find_strings_and_line_comments(
            " SELECT 'this is a \\' -- string ',\"this is also a \\\" -- string \"",
        );

        assert_eq!(2, result.len());
        assert_eq!([8, 33], result[0]);
        assert_eq!([34, 64], result[1]);
    }

    #[test]
    fn find_strings_and_line_hashtag_comment() {
        let result = find_strings_and_line_comments(
            " SELECT 'this is a \\' -- string ',\"this is also a \\\" -- string \" #really",
        );

        assert_eq!(3, result.len());
        assert_eq!([8, 33], result[0]);
        assert_eq!([34, 64], result[1]);
        assert_eq!([65, 72], result[2]);
    }

    #[test]
    fn find_accent() {
        let result = find_strings_and_line_comments(
            "FROM `damiao-project-1.PvhTest.PimExport` pim",
        );

        assert_eq!(1, result.len());
        assert_eq!([5, 41], result[0]);
    }
    
}
