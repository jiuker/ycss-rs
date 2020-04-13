pub trait Repl{
    /*
        GetFileBody() string
        GetOutFileBody() string
        FindClass([]*regexp.Regexp) []string
        GetRegexpCss([]string, *sync.Map, css.Css) *string
        // needZoomKey for rn
        Zoom(css *string, unit string, needZoomUint string, needZoomKey []string, zoom float64) *string
        GetOldCss(*regexp.Regexp) (*string, *string, error)
        Replace(old *string, new *string, pos *string) *string
        Save(newPos *string, old *string) error
        // close file
        Done()
    */
    fn new(path:String)->Self;
    fn get_file_body(&self)->String;
    fn get_class(&self)->Vec<String>;
    fn get_new_css(&self, cls:Vec<String>) ->String;
    fn get_old_css(&self) ->String;
    fn is_same(&self,a:String,b:String)->bool;
    fn write(&self,new_css:String,old_css:String);
}