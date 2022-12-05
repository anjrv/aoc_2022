import scala.io.Source
import scala.collection.mutable.Stack

object Solution_2 {
  def main(args: Array[String]): Unit = {
    val input = Source.fromFile("input.txt").getLines().toArray
    val splitIdx = input.indexOf("");
    var crates =
      Array.fill[Stack[Char]](input(0).length() / 4 + 1)(Stack[Char]())

    for (i <- splitIdx - 2 to 0 by -1) { // Skip split line itself and indexes
      for (j <- 1 to input(i).length() - 1 by 4) {
        val crate = input(i).charAt(j)

        if (crate != ' ') {
          crates(j / 4).push(crate)
        }
      }
    }

    var crane = Stack[Char]()
    for (i <- splitIdx + 1 to input.length - 1) {
      val move = input(i).split(' ')
      val N = move(1).toInt - 1

      for (j <- 0 to N) {
        crane.push(crates(move(3).toInt - 1).pop())
      }

      for (j <- 0 to N) {
        crates(move(5).toInt - 1).push(crane.pop())
      }
    }

    var ans = ""
    crates.foreach(ans += _.pop())
    println(ans)
  }
}
