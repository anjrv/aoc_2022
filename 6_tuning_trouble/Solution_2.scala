import scala.io.Source

object Solution_2 {
  def main(args: Array[String]): Unit = {
    val input = Source.fromFile("input.txt").mkString;
    val numDistinct = 14

    var chars = input.substring(0, numDistinct).toCharArray()

    for (i <- numDistinct to input.length) {
      // Alternatively insert into HashMap and check length
      if (chars.distinct.length == numDistinct) {
        println(i)
        return
      }
    
      chars.update(i % numDistinct, input(i))
    }
  }
}
