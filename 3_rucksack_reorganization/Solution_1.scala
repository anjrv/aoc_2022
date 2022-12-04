import scala.io.Source

object Solution_1 {
  def main(args: Array[String]): Unit = {
    val input = Source.fromFile("input.txt")
    var priority = 0;

    for (line <- input.getLines()) {
      val packs = line.splitAt(line.length / 2)
      val shared = (packs._1 intersect packs._2).charAt(0).toInt

      if (shared > 90)
        priority += shared - 96
      else
        priority += shared - 38
    }

    println(priority)
    input.close
  }
}
