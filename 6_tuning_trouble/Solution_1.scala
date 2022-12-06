import scala.io.Source

object Solution_1 {
  def main(args: Array[String]): Unit = {
    val input = Source.fromFile("input.txt").mkString;
    // Part 1: val numDistinct = 4
    val numDistinct = 14

    var chars = input.substring(0, numDistinct).toCharArray()

    for (i <- numDistinct to input.length) {
      if (chars.distinct.length == numDistinct) {
        println(i)
        return
      }

      chars.update(i % numDistinct, input(i))
    }
  }
}
