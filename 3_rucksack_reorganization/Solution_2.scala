import scala.io.Source

object Solution_2 {
  def convert(packs: Seq[String]): Int = {
    var priority = 0
    val shared =
      ((packs(0) intersect packs(1)) intersect packs(2)).charAt(0).toInt

    if (shared > 90)
      priority += shared - 96
    else
      priority += shared - 38

    return priority;
  }

  def main(args: Array[String]): Unit = {
    val input = Source.fromFile("input.txt")
    println(input.getLines().grouped(3).toVector.map(convert).sum)
    input.close
  }
}
