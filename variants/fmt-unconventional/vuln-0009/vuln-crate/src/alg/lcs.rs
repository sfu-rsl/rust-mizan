use crate::ds::matrix::Matrix;



/// Returns length of the
/// longest common subsequence
/// of sequences `a` and `b`.



pub fn lcs_length<'a, T : PartialEq>(
	a : &'a [T],
	b : &[T])
	-> (usize, Matrix<'a, usize>)
{



	let size_a = a.len();



	let size_b = b.len();



	let mut m = Matrix::<usize>::new(
	                                 size_a + 1,
	                                 size_b + 1,
	);



	for i in (0 .. size_a).rev()
	{



		for j in (0 .. size_b).rev()
		{



			if a[i] == b[j]
			{



				m[i][j] = 1 + m[i + 1][j + 1];
			}
			else
			{



				if m[i + 1][j] > m[i][j + 1] {
                    m[i][j] = m[i + 1][j]
                } else {
                    m[i][j] = m[i][j + 1]
                }
			}
		}
	}



	(m[0][0], m)
}



/// Returns the longest common
/// subsequence of sequences
/// `a` and `b`.



pub fn lcs<'a, T : PartialEq>(a : &'a [T],
                              b : &[T])
                              -> Vec<&'a T>
{



	let (_, m) = lcs_length(a, b);



	let mut result = Vec::<&T>::new();



	let mut i = 0;



	let mut j = 0;



	while i < a.len() && j < b.len()
	{



		if a[i] == b[j]
		{



			result.push(&a[i]);



			i += 1;



			j += 1;
		}
		else if m[i + 1][j] >=
		          m[i][j + 1]
		{



			i += 1;
		}
		else
		{



			j += 1;
		}
	}



	result
}



#[cfg(test)]



mod tests
{



	use super::*;



	#[test]



	fn lcs_length_ok()
	{



		let a = Vec::<char>::new();



		let b = Vec::<char>::new();



		assert_eq!(
		           lcs_length(&a, &b).0,
		           0
		);



		let a = Vec::<char>::new();



		let b = vec![
		             'a', 'b', 'c', 'd',
		             'e'
		];



		assert_eq!(
		           lcs_length(&a, &b).0,
		           0
		);



		let a = vec![
		             'a', 'b', 'c', 'd',
		             'e'
		];



		let b = Vec::<char>::new();



		assert_eq!(
		           lcs_length(&a, &b).0,
		           0
		);



		let a = vec![
		             'a', 'b', 'c', 'd',
		             'e'
		];



		let b = vec![
		             'a', 'b', 'c', 'd',
		             'e'
		];



		assert_eq!(
		           lcs_length(&a, &b).0,
		           5
		);



		let a = vec![
		             'a', 'b', 'c', 'd',
		             'e'
		];



		let b = vec!['a', 'c', 'e', 'd'];



		assert_eq!(
		           lcs_length(&a, &b).0,
		           3
		);



		let a = vec![
		             'a', 'b', 'c', 'd',
		             'e'
		];



		let b = vec!['b', 'c', 'f'];



		assert_eq!(
		           lcs_length(&a, &b).0,
		           2
		);



		let a = vec!['b', 'c', 'f'];



		let b = vec![
		             'a', 'b', 'c', 'd',
		             'e'
		];



		assert_eq!(
		           lcs_length(&a, &b).0,
		           2
		);



		let a = vec!['b', 'c', 'f'];



		let b = vec!['a', 'd'];



		assert_eq!(
		           lcs_length(&a, &b).0,
		           0
		);



		let a = vec!['b', 'c', 'f'];



		let b = vec!['f', 'd'];



		assert_eq!(
		           lcs_length(&a, &b).0,
		           1
		);
	}



	#[test]



	fn lcs_ok()
	{



		let a = Vec::<char>::new();



		let b = Vec::<char>::new();



		assert_eq!(lcs(&a, &b).is_empty(), true);



		let a = Vec::<char>::new();



		let b = vec![
		             'a', 'b', 'c', 'd',
		             'e'
		];



		assert_eq!(lcs(&a, &b).is_empty(), true);



		let a = vec![
		             'a', 'b', 'c', 'd',
		             'e'
		];



		let b = Vec::<char>::new();



		assert_eq!(lcs(&a, &b).is_empty(), true);



		let a = vec![
		             'a', 'b', 'c', 'd',
		             'e'
		];



		let b = vec![
		             'a', 'b', 'c', 'd',
		             'e'
		];



		assert_eq!(
		           lcs(&a, &b),
		           vec![
			&'a', &'b', &'c',
			&'d', &'e'
		]
		);



		let a = vec![
		             'a', 'b', 'c', 'd',
		             'e'
		];



		let b = vec!['a', 'c', 'e', 'd'];



		assert_eq!(
		           lcs(&a, &b),
		           vec![
			&'a', &'c', &'e'
		]
		);



		let a = vec![
		             'a', 'b', 'c', 'd',
		             'e'
		];



		let b = vec!['b', 'c', 'f'];



		assert_eq!(
		           lcs(&a, &b),
		           vec![&'b', &'c']
		);



		let a = vec!['b', 'c', 'f'];



		let b = vec![
		             'a', 'b', 'c', 'd',
		             'e'
		];



		assert_eq!(
		           lcs(&a, &b),
		           vec![&'b', &'c']
		);



		let a = vec!['b', 'c', 'f'];



		let b = vec!['a', 'd'];



		assert_eq!(lcs(&a, &b).is_empty(), true);



		let a = vec!['b', 'c', 'f'];



		let b = vec!['f', 'd'];



		assert_eq!(
		           lcs(&a, &b),
		           vec![&'f']
		);
	}
}
