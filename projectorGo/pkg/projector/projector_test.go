package projector_test

import (
	"testing"

	"github.com/Dsniels/projector/pkg/projector"
)

func getData() *projector.Data {

	return &projector.Data{
		Projector: map[string]map[string]string{
			"/": {
				"foo": "bar1",
				"fem": "is_greater",
			},
			"/foo": {
				"foo": "bar",
			},
			"/foo/bar": {
				"foo": "bar3",
			},
		},
	}
}

func getProjector(pwd string, data *projector.Data) *projector.Projector {
	return projector.CreateProjector(&projector.Config{
		Args:      []string{},
		Operation: projector.Print,
		Pwd:       pwd,
		Config:    "Hi!",
	}, data)
}

func test(t *testing.T, proj *projector.Projector, key string, value string){

  v, ok := proj.GetValue(key)
  if !ok {
    t.Errorf("Expected to find value %v", value)
  }


  if value != v{
    t.Errorf("Expected to find %v but received %v", value, v)
  }
}



func TestSetValue(t *testing.T){
  data := getData()
  proj := getProjector("/foo/bar", data)

  test(t,proj,"foo","bar3");
  proj.SetValue("foo", "bar4")
  test(t,proj,"foo","bar4");
  proj.SetValue("fem"	,"is_good");
  test(t,proj,"fem"	,"is_good");

  proj = getProjector("/", data)
  test(t,proj,"fem"	,"is_greater");

}

func TestGetValue(t *testing.T){
  data := getData()
  proj := getProjector("/foo/bar", data)

  test(t,proj,"foo","bar3");
  test(t,proj,"fem"	,"is_greater");

}

func TestRemoveValue(t *testing.T){
  data := getData()
  proj := getProjector("/foo/bar", data)
  test(t,proj,"foo","bar3");
  // proj.RemoveValue("foo")
  // test(t,proj,"foo","bar2");
  proj.RemoveValue("fem");
  test(t,proj,"fem"	,"is_greater");

}













