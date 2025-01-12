import Prelude hiding (Bool, all, and, even, map, replicate, succ)

newtype Bool = Bool (forall r. r -> r -> r)

true = Bool $ \x y -> x
false = Bool $ \x y -> y

and (Bool x) y = x y false

instance Show Bool where
  show (Bool x) = x "true" "false"

newtype Nat = Nat (forall r. (Nat -> r) -> r -> r)

succ n = Nat $ \s z -> s n
zero = Nat $ \s z -> z

add a (Nat b) = b (addS a) a
addS a b = succ (add a b)

mul a (Nat b) = b (mulS a) zero
mulS a b = add a (mul a b)

even (Nat n) = n evenS true
evenS (Nat n) = n even false

data List a = List (forall r. (a -> List a -> r) -> r -> r)

cons x xs = List $ \c n -> c x xs
nil = List $ \c n -> n

map f (List xs) = xs (mapCons f) nil
mapCons f x xs = cons (f x) (map f xs)

fold f z (List xs) = xs (foldCons f z) z
foldCons f z x xs = fold f (f z x) xs

replicate (Nat n) x = n (replicateCons x) nil
replicateCons x n = cons x (replicate n x)

all f xs = fold and true (map f xs)

n0 = zero
n1 = succ n0
n2 = succ n1
n4 = mul n2 n2
n16 = mul n4 n4
n256 = mul n16 n16
n1024 = mul n256 n4

numbers = replicate n1024 n1024

main = print $ all even numbers
