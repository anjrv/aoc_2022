import scala.io.Source

object Solution_2 {
  def in(x: Int, a: Int, b: Int): Boolean = {
    return x >= a && x <= b
  }

  def main(args: Array[String]): Unit = {
    val input = Source.fromFile("input.txt")
    var sum = 0

    for (line <- input.getLines()) {
      val sections = line.split("[-,]").map(_.toInt)

      if (
        in(sections(0), sections(2), sections(3)) ||
        in(sections(1), sections(2), sections(3)) ||
        in(sections(2), sections(0), sections(1)) ||
        in(sections(3), sections(0), sections(1))
      ) {
        sum += 1;
      }
    }

    println(sum)

    input.close
  }
}
