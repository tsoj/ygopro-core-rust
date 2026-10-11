//! Magikey Duo
use super::prelude::*;

pub const ID: u32 = 51510279;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MAGIKEY],
    ..ScriptInfo::DEFAULT
};

/// `fparams={fusfilter=aux.FilterBoolFunction(Card.IsSetCard,SET_MAGIKEY),sumpos=POS_FACEUP_DEFENSE}`
fn fparams() -> fusion::SummonEff {
    fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_MAGIKEY as u16]))),
        sumpos: Some(POS_FACEUP_DEFENSE as u8),
        ..Default::default()
    }
}
/// `rparams={filter=aux.FilterBoolFunction(Card.IsSetCard,SET_MAGIKEY),lvtype=RITPROC_GREATER,sumpos=POS_FACEUP_DEFENSE}`
fn rparams() -> ritual::RitualProc {
    ritual::RitualProc {
        filter: Some(std::sync::Arc::new(|d: &mut Duel, c: CardId| card::is_set_card(d, c, &[SET_MAGIKEY as u16]))),
        lvtype: RITPROC_GREATER,
        sumpos: Some(POS_FACEUP_DEFENSE as u8),
        ..Default::default()
    }
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation(Fusion.SummonEffTG(fparams),Fusion.SummonEffOP(fparams),Ritual.Target(rparams),Ritual.Operation(rparams)))
    let fustg = fusion::summon_eff_tg(d, fparams());
    let fusop = fusion::summon_eff_op(d, fparams());
    let rittg = ritual::target(d, rparams());
    let ritop = ritual::operation(d, rparams());
    effect::set_operation(d, e1, Some(operation(move |d, a| op_body(d, a, &fustg, &fusop, &rittg, &ritop))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_MAGIKEY) or c:IsType(TYPE_NORMAL)) and c:IsMonster() and c:IsAbleToHand()
    (card::is_set_card(d, c, &[SET_MAGIKEY as u16]) || card::is_type(d, c, TYPE_NORMAL)) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, 0, 0);
    true
}
/// The arguments of a Lua call of the effect functions (`chk` = 0 for the target check).
fn args(a: &EffArgs, chk: Option<i64>) -> Vec<Value> {
    let mut v = vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    if let Some(chk) = chk {
        v.push(Value::Int(chk));
    }
    v
}
/// `s.operation(fustg,fusop,rittg,ritop)`: the returned `function(e,tp,eg,ep,ev,re,r,rp)`
fn op_body(d: &mut Duel, a: &EffArgs, fustg: &LuaFn, fusop: &LuaFn, rittg: &LuaFn, ritop: &LuaFn) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)~=0 then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) && duel::send_to_hand(d, tc, None, REASON_EFFECT, None) != 0 {
            // local fus=fustg(e,tp,eg,ep,ev,re,r,rp,0)
            let fus = fustg.call(d, &args(a, Some(0))).truthy();
            // local rit=rittg(e,tp,eg,ep,ev,re,r,rp,0)
            let rit = rittg.call(d, &args(a, Some(0))).truthy();
            // if fus or rit then
            if fus || rit {
                // local sel={}
                // table.insert(sel,aux.Stringid(id,1))
                let mut sel: Vec<u64> = vec![aux::stringid(ID, 1)];
                // if fus then table.insert(sel,aux.Stringid(id,2)) end
                if fus {
                    sel.push(aux::stringid(ID, 2));
                }
                // if rit then table.insert(sel,aux.Stringid(id,3)) end
                if rit {
                    sel.push(aux::stringid(ID, 3));
                }
                // local res=Duel.SelectOption(tp,false,table.unpack(sel))
                let res = duel::select_option(d, a.tp, false, &sel);
                // if res==0 then return end
                if res == 0 {
                    return;
                }
                // Duel.BreakEffect()
                duel::break_effect(d);
                // if res==1 and fus then
                if res == 1 && fus {
                    // Duel.Hint(HINT_OPSELECTED,tp,aux.Stringid(id,2))
                    duel::hint(d, HINT_OPSELECTED as u8, a.tp, aux::stringid(ID, 2));
                    // fusop(e,tp,eg,ep,ev,re,r,rp)
                    fusop.call(d, &args(a, None));
                } else {
                    // Duel.Hint(HINT_OPSELECTED,tp,aux.Stringid(id,3))
                    duel::hint(d, HINT_OPSELECTED as u8, a.tp, aux::stringid(ID, 3));
                    // ritop(e,tp,eg,ep,ev,re,r,rp)
                    ritop.call(d, &args(a, None));
                }
            }
        }
    }
}
