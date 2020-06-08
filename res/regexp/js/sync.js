/*
 * 页面上只要出现 class=""就会被该js抓取，这个抓取的原理是获取网页实现的
 * (空格)class="" c-class="asd{asd{p-0010 p-10}}"类似的都会被争取处理
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
            alert("没有符合插入css的条件,个数不足！");
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
                    alert("没有符合插入css的条件");
                    break;
                case 1:
                    window.addEventListener("keypress",function(e){ //c复制
                        if(e.keyCode===99){
                            if(typeof HTMLCOPY !='function'){
                                var content = "没有内容";
                                var aux = document.createElement("textarea");
                                // 获取复制内容
                                aux.value = content;
                                // 将元素插入页面进行调用
                                document.body.appendChild(aux);
                                // 复制内容
                                aux.select();
                                document.execCommand("selectAll");
                                // 将内容复制到剪贴板
                                document.execCommand("cut");
                                //    // 删除创建元素
                                document.body.removeChild(aux);
                            }else{
                                HTMLCOPY();
                            }
                        }
                    });
                    window.HTMLCOPY = function(){
                        var content = document.querySelectorAll("style")[1].innerHTML;
                        var aux = document.createElement("textarea");
                        // 获取复制内容
                        aux.value = content;
                        // 将元素插入页面进行调用
                        document.body.appendChild(aux);
                        // 复制内容
                        aux.select();
                        document.execCommand("selectAll");
                        // 将内容复制到剪贴板
                        document.execCommand("cut");
                        //    // 删除创建元素
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
         *根据url来初始化需要显示的页面
         */
        _this.initHtml=function(url,cb){
            var obj=new XMLHttpRequest();
            obj.open("GET",url,true);
            obj.onreadystatechange = function() {
                if (obj.readyState == 4 && obj.status == 200 || obj.status == 304) { // readyState == 4说明请求已完成
                    _this._html+=obj.responseText+" ";
                    cb();
                }
            };
            obj.send();
        };
    /*
     *初始化cssClass数组
     *第一种 class="any class",第二种 c-class="any class"
     */
    _this.initClassArray=function(){
        /*
         *handle class="any class"
         */
        //match class="any class",
        // map 为es5映射，对map里面的进行修改拷贝数组也会修改。
        //forEach 为es5遍历，对里面的进行修改不会对元素进行修改。
        //filter 为es5过滤筛选
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
     * 正常的class会过滤掉不存在的属性。
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
     * common性质的class
     * */
    _this.initCommonClassArray=function(){
        var classMatch = _this._html.match(/c-class="([^"]+)"/g)||[];
        classMatch = classMatch.map(function(v,i){
            return v.replace('c-class="',"").replace('"',"")
        });
        _this.commonClassList=classMatch;
    };
    /*
     把common css输入到ouput
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
                        //去除掉原来的table
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
                        //把无用数组过滤掉
                        return v4.indexOf("{")!=-1;
                    }).map(function(v5,i5){
                        //把数组改成可渲染的数组
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
                    //这是px
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
                    //这是rem
                    var baseNum=parseFloat(v.match(/<style[^>]*>/)[0].match(/rem="([^"]+)"/g)[0].replace(/[rem="|"]/g,""));
                    css=css.replace(/\d{1,10}px/g,function(a){
                        var d=parseFloat(a.replace("px",""));
                        if (d==0){
                            return '0';
                        }
                        //这个小于等于1就默认不转,防止1px的时候会产生意外
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
        //没有找到可以插入的style，就默认插入第二个
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
      rp: new RegExp(/^com-tt-(\d{1,2})-(\d{1,2})-(\d{3,3})(\d{3,3})(\d{3,3})(\d{2,2})$/), //宽，高，颜色
      rep: "width:0;height:0;border-width:0 $1px $2px $1px  ;border-style:solid;border-color: transparent  transparent rbga($3,$4,$5,$6) transparent;"
   })
*/
HTMLCSS.prototype.initRegExp=function(){
    //insertHere
}