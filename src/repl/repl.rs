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
}