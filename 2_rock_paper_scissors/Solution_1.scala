import scala.io.Source

object Solution_1 {
  def main(args: Array[String]): Unit = {
    val input = Source.fromFile("input.txt")
    var score = 0;

    for (line <- input.getLines()) {
      val opp = line.charAt(0) - 64
      val you = line.charAt(2) - 87
      score += you

      if (you == opp)
        score += 3
      else if (you - opp == -2 || (you - opp) % 2 > 0)
        score += 6
    }

    println(score)
    input.close
  }
}
