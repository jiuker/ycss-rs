/*
 * 椤甸潰涓婂彧瑕佸嚭鐜� class=""灏变細琚js鎶撳彇锛岃繖涓姄鍙栫殑鍘熺悊鏄幏鍙栫綉椤靛疄鐜扮殑
 * (绌烘牸)class="" c-class="asd{asd{p-0010 p-10}}"绫讳技鐨勯兘浼氳浜夊彇澶勭悊
 * */
var styleFirst = document.querySelector("style");
if(styleFirst != null) {
    styleFirst.innerHTML = styleFirst.innerHTML + "@media screen and (max-width: 640px){::-webkit-scrollbar{width: 0px;height:0px;}}"
};
function HTMLCSS(){
    var _this=this;
    _this._html="";
    _this.classList=[];
    _this.commonClassList=[];
    _this.regexps=[];
    _this.output=[];
    _this.init=function(url){
        var styles=document.querySelectorAll("style");
        if(styles.length>=2){
        }else{
            alert("娌℃湁绗﹀悎鎻掑叆css鐨勬潯浠�,涓暟涓嶈冻锛�");
            return
        }
        _this.initHtml(url,function(){
            _this.initRegExp();
            _this.initClassArray();
            _this.classArrayToOutput();
            _this.initCommonClassArray();
            _this.commonClassArrayToOutput();
            switch (_this.findStyleElementToInsert(_this.output.join("").replace(/}/g,"}\r\n").replace(/\n /g,""))){
                case 0:
                    alert("娌℃湁绗﹀悎鎻掑叆css鐨勬潯浠�");
                    break;
                case 1:
                    window.addEventListener("keypress",function(e){ //c澶嶅埗
                        if(e.keyCode===99){
                            if(typeof HTMLCOPY !='function'){
                                var content = "娌℃湁鍐呭";
                                var aux = document.createElement("textarea");
                                // 鑾峰彇澶嶅埗鍐呭
                                aux.value = content;
                                // 灏嗗厓绱犳彃鍏ラ〉闈㈣繘琛岃皟鐢�
                                document.body.appendChild(aux);
                                // 澶嶅埗鍐呭
                                aux.select();
                                document.execCommand("selectAll");
                                // 灏嗗唴瀹瑰鍒跺埌鍓创鏉�
                                document.execCommand("cut");
                                //    // 鍒犻櫎鍒涘缓鍏冪礌
                                document.body.removeChild(aux);
                            }else{
                                HTMLCOPY();
                            }
                        }
                    });
                    window.HTMLCOPY = function(){
                        var content = document.querySelectorAll("style")[1].innerHTML;
                        var aux = document.createElement("textarea");
                        // 鑾峰彇澶嶅埗鍐呭
                        aux.value = content;
                        // 灏嗗厓绱犳彃鍏ラ〉闈㈣繘琛岃皟鐢�
                        document.body.appendChild(aux);
                        // 澶嶅埗鍐呭
                        aux.select();
                        document.execCommand("selectAll");
                        // 灏嗗唴瀹瑰鍒跺埌鍓创鏉�
                        document.execCommand("cut");
                        //    // 鍒犻櫎鍒涘缓鍏冪礌
                        document.body.removeChild(aux);
                    }
                    document.querySelector('body').insertAdjacentHTML('afterBegin','<div" style="width: 40px;height: 40px;background-color: rgba(255,0,0,0.5);position: fixed;left: 0;top: 0;z-index:111111111;"></div>')
                    console.log(window.location.href+"->>>>>>"+"need fixed!")
                    break;
                case 2:
                    console.log(window.location.href+"->>>>>>"+"dont need fixed!")
                    break;
                default:
                    break;
            }
        });
    },
        /*
         *鏍规嵁url鏉ュ垵濮嬪寲闇€瑕佹樉绀虹殑椤甸潰
         */
        _this.initHtml=function(url,cb){
            var obj=new XMLHttpRequest();
            obj.open("GET",url,true);
            obj.onreadystatechange = function() {
                if (obj.readyState == 4 && obj.status == 200 || obj.status == 304) { // readyState == 4璇存槑璇锋眰宸插畬鎴�
                    _this._html+=obj.responseText+" ";
                    cb();
                }
            };
            obj.send();
        };
    /*
     *鍒濆鍖朿ssClass鏁扮粍
     *绗竴绉� class="any class",绗簩绉� c-class="any class"
     */
    _this.initClassArray=function(){
        /*
         *handle class="any class"
         */
        //match class="any class",
        // map 涓篹s5鏄犲皠锛屽map閲岄潰鐨勮繘琛屼慨鏀规嫹璐濇暟缁勪篃浼氫慨鏀广€�
        //forEach 涓篹s5閬嶅巻锛屽閲岄潰鐨勮繘琛屼慨鏀逛笉浼氬鍏冪礌杩涜淇敼銆�
        //filter 涓篹s5杩囨护绛涢€�
        var classMatch = _this._html.match(/ class="([^"]+)"/g)||[];
        classMatch = classMatch.map(function(v,i){
            return v.replace(' class="',"").replace('"',"")
        });
        classMatch.forEach(function(v,i){
            var splitBySingalClass = v.split(/ /g).filter(function(v1,i1){
                if (v1==""){
                    return false;
                }else{
                    return true;
                }
            });
            splitBySingalClass.forEach(function(v1,i1,arr){
                if (_this.classList.indexOf(v1)==-1){
                    _this.classList.push(v1)
                }
            })
        });
    };
    /*
     * 姝ｅ父鐨刢lass浼氳繃婊ゆ帀涓嶅瓨鍦ㄧ殑灞炴€с€�
     * */
    _this.classArrayToOutput=function(){
        _this.output=_this.classList.map(function(v,i){
            try{
                _this.regexps.forEach(function(v1,i1){
                    if(v.match(v1.rp) != null) {
                        throw "." + v + "{" + v.replace(v1.rp, v1.rep) + "}"
                    }
                })
            }catch(e){
                return e
            }
            return v
        }).filter(function(v,i){
            return v.indexOf("{")!=-1;
        });
    };
    /*
     * common鎬ц川鐨刢lass
     * */
    _this.initCommonClassArray=function(){
        var classMatch = _this._html.match(/c-class="([^"]+)"/g)||[];
        classMatch = classMatch.map(function(v,i){
            return v.replace('c-class="',"").replace('"',"")
        });
        _this.commonClassList=classMatch;
    };
    /*
     鎶奵ommon css杈撳叆鍒皁uput
     * */
    _this.commonClassArrayToOutput=function(){
        var data=_this.commonClassList.map(function(v,i){
            /*
             to find base{} string incloud '{}'
             * */
            var baseCssClass="";
            var start=false;
            for(var i=0;i<v.length;i++){
                if(v[i]=="{"){
                    baseCssClass=""
                    start=true;
                }
                if(start){
                    baseCssClass+=v[i]
                }
                if(v[i]=="}"){
                    start=false;
                    var base=baseCssClass.replace(/{|}/g,"").split(/ /g).filter(function(v1,i1){
                        return v1!=""
                    }).map(function(v2p,i2p){
                        //鍘婚櫎鎺夊師鏉ョ殑table
                        return v2p.replace(/\t/g,"")
                    }).map(function(v2,i2){
//                      console.log(v2)
                        try{
                            _this.regexps.forEach(function(v3,i3){
                                if(v2.match(v3.rp) != null) {
//                                  console.log("{"+v2.replace(v3.rp, v3.rep)+"}")
                                    throw  "{"+v2.replace(v3.rp, v3.rep)+"}"
                                }
                            })
                        }catch(e){
                            return e
                        }
                        return v2
                    }).filter(function(v4,i4){
                        //鎶婃棤鐢ㄦ暟缁勮繃婊ゆ帀
                        return v4.indexOf("{")!=-1;
                    }).map(function(v5,i5){
                        //鎶婃暟缁勬敼鎴愬彲娓叉煋鐨勬暟缁�
                        return v5.replace(/{|}/g,"")
                    }).join("");
                    v=v.replace(baseCssClass,"{"+base+"}")
                }
            }
            return v
        });
        this.output=_this.output.concat(data);
    }
    _this.findStyleElementToInsert=function(css){
        try{
            _this._html.match(/<style[^<]+<\/style>/g).forEach(function(v,i){
                if (v.match(/<style[^>]*>/)[0].match(/px="/)!=null){
                    //杩欐槸px
                    var baseNum=parseFloat(v.match(/<style[^>]*>/)[0].match(/px="([^"]+)"/g)[0].replace(/px="|"]/g,""));
                    css=css.replace(/\d{1,10}px/g,function(a){
                        var d=parseInt(a.replace("px",""));
                        if (d==0){
                            return '0';
                        }
                        if(d<=2){
                            return d+"px";
                        }
                        return d/baseNum+"px"
                    });
                    if (!_this.findStyleElementIsTheSame(v,css)){
                        document.querySelectorAll("style")[i].innerHTML=css;
                        throw 1
                    }else{
                        throw 2
                    }
                }
                if (v.match(/<style[^>]*>/)[0].match(/rem="/)!=null){
                    //杩欐槸rem
                    var baseNum=parseFloat(v.match(/<style[^>]*>/)[0].match(/rem="([^"]+)"/g)[0].replace(/[rem="|"]/g,""));
                    css=css.replace(/\d{1,10}px/g,function(a){
                        var d=parseFloat(a.replace("px",""));
                        if (d==0){
                            return '0';
                        }
                        //杩欎釜灏忎簬绛変簬1灏遍粯璁や笉杞�,闃叉1px鐨勬椂鍊欎細浜х敓鎰忓
                        if(d<=1){
                            return d+"px";
                        }
                        return d/baseNum+"rem"
                    });
                    if (!_this.findStyleElementIsTheSame(v,css)){
                        document.querySelectorAll("style")[i].innerHTML=css;
                        throw 1
                    }else{
                        throw 2
                    }
                }
            })
        }catch(e){
            return e
        }
        //娌℃湁鎵惧埌鍙互鎻掑叆鐨剆tyle锛屽氨榛樿鎻掑叆绗簩涓�
        if (_this._html.match(/<style[^<]+<\/style>/g).length==2){
            var v=_this._html.match(/<style[^<]+<\/style>/g)[1];
            var baseNum=1;
            css=css.replace(/\d{1,10}px/g,function(a){
                var d=parseInt(a.replace("px",""));
                if (d==0){
                    return '0';
                }
                if(d<=2){
                    return d+"px";
                }
                return d/baseNum+"px"
            });
            if (!_this.findStyleElementIsTheSame(v,css)){
                document.querySelectorAll("style")[1].innerHTML=css;
                return 1
            }else{
                return 2
            }
        }
        return 0
    }
    _this.findStyleElementIsTheSame=function(old,css){
        old=old.replace(/<style[^>]*/,"").replace(/<\/style>/,"")
        var data='0123456789-qwertyuiopasdfghjkl:;zxcvbnm,#"'
        for(var i = 0; i < data.length; i++) {
            var r = eval("/" + data[i] + "/g")
            if((old.match(r) || []).length != (css.match(r) || []).length) {
                return false
            }
        }
        return true;
    }
}
setTimeout(function(){
    var htmlcssObj=new HTMLCSS();
    htmlcssObj.init(window.location.href);
});
/*
*  this.regexps.push({
      rp: new RegExp(/^com-tt-(\d{1,2})-(\d{1,2})-(\d{3,3})(\d{3,3})(\d{3,3})(\d{2,2})$/), //瀹斤紝楂橈紝棰滆壊
      rep: "width:0;height:0;border-width:0 $1px $2px $1px  ;border-style:solid;border-color: transparent  transparent rbga($3,$4,$5,$6) transparent;"
   })
*/
HTMLCSS.prototype.initRegExp=function(){
    this.regexps.push({
        rp:new RegExp(/^o-(\d{1,3})-([A-Z|a-z|0-9]{3,6})$/),
        rep:'outline:#$2dotted$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^ls-([0-9]{1,3})$/),
        rep:'letter-spacing:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^fd-c$/),
        rep:'flex-direction:column;',
    })
    this.regexps.push({
        rp:new RegExp(/^bw-(\d{1,3})$/),
        rep:'border-width:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^w-(\d{1,3})$/),
        rep:'width:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^b-(\d{1,3})$/),
        rep:'bottom:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^jc-c$/),
        rep:'justify-content:center;',
    })
    this.regexps.push({
        rp:new RegExp(/^ml-([-|0-9]{1,4})$/),
        rep:'margin-left:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^bc-(\d{3,3})-(\d{3,3})-(\d{3,3})$/),
        rep:'background-color:rgb($1,$2,$3);',
    })
    this.regexps.push({
        rp:new RegExp(/^bl-(\d{1,3})-([A-Z|a-z|0-9]{3,6})$/),
        rep:'border-left:$1pxsolid#$2;',
    })
    this.regexps.push({
        rp:new RegExp(/^ai-c$/),
        rep:'align-items:center;',
    })
    this.regexps.push({
        rp:new RegExp(/^c-([A-Z|a-z|0-9]{3,6})$/),
        rep:'color:#$1;',
    })
    this.regexps.push({
        rp:new RegExp(/^w-(\d{1,3})-vw$/),
        rep:'width:$1vw;',
    })
    this.regexps.push({
        rp:new RegExp(/^f-(\d{1,3})$/),
        rep:'-webkit-flex:$1;flex:$1;',
    })
    this.regexps.push({
        rp:new RegExp(/^ta-e$/),
        rep:'text-align:end;',
    })
    this.regexps.push({
        rp:new RegExp(/^td-n$/),
        rep:'text-decoration:none',
    })
    this.regexps.push({
        rp:new RegExp(/^ta-c$/),
        rep:'text-align:center;',
    })
    this.regexps.push({
        rp:new RegExp(/^maxh(\d{1,3})$/),
        rep:'max-height:$1%;',
    })
    this.regexps.push({
        rp:new RegExp(/^br-(\d{1,3})$/),
        rep:'border-radius:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^m-(\d{2,2})(\d{2,2})$/),
        rep:'margin:$1px$2px;',
    })
    this.regexps.push({
        rp:new RegExp(/^t-(\d{1,3})$/),
        rep:'top:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^o-a$/),
        rep:'overflow:auto;',
    })
    this.regexps.push({
        rp:new RegExp(/^fw-([A-Z|a-z|0-9]{3,6})$/),
        rep:'font-weight:$1;',
    })
    this.regexps.push({
        rp:new RegExp(/^minh-(\d{1,3})$/),
        rep:'min-height:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^bt-(\d{1,3})-([A-Z|a-z|0-9]{3,6})$/),
        rep:'border-top:$1pxsolid#$2;',
    })
    this.regexps.push({
        rp:new RegExp(/^br-nr$/),
        rep:'background-repeat:no-repeat;',
    })
    this.regexps.push({
        rp:new RegExp(/^fs-(\d{1,3})$/),
        rep:'font-size:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^p-a$/),
        rep:'position:absolute;',
    })
    this.regexps.push({
        rp:new RegExp(/^p-f$/),
        rep:'position:fixed;',
    })
    this.regexps.push({
        rp:new RegExp(/^maxw(\d{1,3})$/),
        rep:'max-width:$1%;',
    })
    this.regexps.push({
        rp:new RegExp(/^d-b$/),
        rep:'display:block;',
    })
    this.regexps.push({
        rp:new RegExp(/^o-h-(\d{1,2})$/),
        rep:'display:-webkit-box;-webkit-box-orient:vertical;-webkit-line-clamp:$1;overflow:hidden;',
    })
    this.regexps.push({
        rp:new RegExp(/^bs-cover$/),
        rep:'background-size:cover;',
    })
    this.regexps.push({
        rp:new RegExp(/^b-(\d{1,3})-([A-Z|a-z|0-9]{3,6})$/),
        rep:'border:$1pxsolid#$2;',
    })
    this.regexps.push({
        rp:new RegExp(/^d-i$/),
        rep:'display:inline;',
    })
    this.regexps.push({
        rp:new RegExp(/^test-red$/),
        rep:'color:@{red}',
    })
    this.regexps.push({
        rp:new RegExp(/^fw-w$/),
        rep:'flex-wrap:wrap;',
    })
    this.regexps.push({
        rp:new RegExp(/^o-n$/),
        rep:'outline:none;',
    })
    this.regexps.push({
        rp:new RegExp(/^pt-([-|0-9]{1,4})$/),
        rep:'padding-top:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^p-r$/),
        rep:'position:relative;',
    })
    this.regexps.push({
        rp:new RegExp(/^h-(\d{1,3})-vh$/),
        rep:'height:$1vh;',
    })
    this.regexps.push({
        rp:new RegExp(/^maxh-(\d{1,3})$/),
        rep:'max-height:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^pr-([-|0-9]{1,4})$/),
        rep:'padding-right:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^bp-c$/),
        rep:'background-position:center;',
    })
    this.regexps.push({
        rp:new RegExp(/^fw-nw$/),
        rep:'flex-wrap:nowrap;',
    })
    this.regexps.push({
        rp:new RegExp(/^pb-([-|0-9]{1,4})$/),
        rep:'padding-bottom:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^bs-(\d{1,3})-(\d{1,3})$/),
        rep:'background-size:$1px$2px;',
    })
    this.regexps.push({
        rp:new RegExp(/^minw-(\d{1,3})$/),
        rep:'min-width:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^mb-([-|0-9]{1,4})$/),
        rep:'margin-bottom:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^w(\d{1,3})$/),
        rep:'width:$1%;',
    })
    this.regexps.push({
        rp:new RegExp(/^maxw-(\d{1,3})$/),
        rep:'max-width:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^h(\d{1,3})$/),
        rep:'height:$1%;',
    })
    this.regexps.push({
        rp:new RegExp(/^ta-l$/),
        rep:'text-align:left;',
    })
    this.regexps.push({
        rp:new RegExp(/^va-m$/),
        rep:'vertical-align:middle;',
    })
    this.regexps.push({
        rp:new RegExp(/^zi-(\d{1,6})$/),
        rep:'z-index:$1;',
    })
    this.regexps.push({
        rp:new RegExp(/^d-ib$/),
        rep:'display:inline-block;',
    })
    this.regexps.push({
        rp:new RegExp(/^l-(\d{1,3})$/),
        rep:'left:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^bc-([A-Z|a-z|0-9]{6,6})$/),
        rep:'background-color:#$1;',
    })
    this.regexps.push({
        rp:new RegExp(/^br-(\d{1,3})-([A-Z|a-z|0-9]{3,6})$/),
        rep:'border-right:$1pxsolid#$2;',
    })
    this.regexps.push({
        rp:new RegExp(/^jc-fe$/),
        rep:'justify-content:flex-end;',
    })
    this.regexps.push({
        rp:new RegExp(/^d-f$/),
        rep:'display:-webkit-flex;display:flex;',
    })
    this.regexps.push({
        rp:new RegExp(/^c-p$/),
        rep:'cursor:pointer;',
    })
    this.regexps.push({
        rp:new RegExp(/^p-(\d{2,2})(\d{2,2})$/),
        rep:'padding:$1px$2px;',
    })
    this.regexps.push({
        rp:new RegExp(/^bb-(\d{1,3})-([A-Z|a-z|0-9]{3,6})$/),
        rep:'border-bottom:$1pxsolid#$2;',
    })
    this.regexps.push({
        rp:new RegExp(/^mr-([-|0-9]{1,4})$/),
        rep:'margin-right:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^ta-s$/),
        rep:'text-align:start;',
    })
    this.regexps.push({
        rp:new RegExp(/^pl-([-|0-9]{1,4})$/),
        rep:'padding-left:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^mt-([-|0-9]{1,4})$/),
        rep:'margin-top:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^fd-r$/),
        rep:'flex-direction:row;',
    })
    this.regexps.push({
        rp:new RegExp(/^lh-([0-9]{1,3})$/),
        rep:'line-height:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^h-(\d{1,3})$/),
        rep:'height:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^r-(\d{1,3})$/),
        rep:'right:$1px;',
    })
    this.regexps.push({
        rp:new RegExp(/^ta-r$/),
        rep:'text-align:right;',
    })

}