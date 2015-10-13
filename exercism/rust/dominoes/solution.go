package main

import (
	"fmt"

	"github.com/alonsovidales/go_graph"
)

func main() {
	// gr := graphs.GetUnWeightGraph([][]uint64{
	// 	[]uint64{1, 2},
	// 	[]uint64{3, 1},
	// 	[]uint64{2, 3},
	// }, true)

	gr := graphs.GetUnWeightGraph([][]uint64{
		[]uint64{1, 2},
		[]uint64{5, 3},
		[]uint64{3, 1},
		[]uint64{1, 2},
		[]uint64{2, 4},
		[]uint64{1, 6},
		[]uint64{2, 3},
		[]uint64{3, 4},
		[]uint64{5, 6},
	}, false)

	tour, possible := gr.EulerianCycle(0)
	fmt.Println(tour, possible)
}
