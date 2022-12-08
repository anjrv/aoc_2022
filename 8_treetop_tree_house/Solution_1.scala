import scala.io.Source

object Solution_1 {
  def main(args: Array[String]): Unit = {
    val input = Source
      .fromFile("input.txt")
      .getLines()
      .map(_.map(_.asDigit).toArray)
      .toArray
  }
}
