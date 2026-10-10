//! 暗黒界の龍神 グラファ
//! Grapha, Dragon Lord of Dark World
use super::prelude::*;

pub const ID: u32 = 34230233;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DARK_WORLD], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,tp)`
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DARK_WORLD) and not c:IsCode(id) and c:IsAbleToHandAsCost()
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_DARK_WORLD as u16])
        && !card::is_code(d, c, &[ID])
        && card::is_able_to_hand_as_cost(d, c)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE,0,nil)
    let rg = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local eff={c:GetCardEffect(EFFECT_NECRO_VALLEY)}
    let eff = card::get_card_effect(d, c, EFFECT_NECRO_VALLEY);
    // for _,te in ipairs(eff) do
    for te in eff {
        // local op=te:GetOperation()
        let op = effect::get_operation(d, te);
        // if not op or op(e,c) then return false end
        match op {
            None => return false,
            Some(op) => {
                if op.call(d, &[Value::Effect(e), Value::Card(c)]).truthy() {
                    return false;
                }
            }
        }
    }
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return aux.SelectUnselectGroup(rg,e,tp,1,1,aux.ChkfMMZ(1),0)
    let chk = aux::chkf_mmz(1);
    matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), Some(&chk), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local c=e:GetHandler()
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE,0,nil)
    let rg = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,1,1,aux.ChkfMMZ(1),1,tp,HINTMSG_RTOHAND,nil,nil,true)
    let chk = aux::chkf_mmz(1);
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), Some(&chk), 1, tp, Some(HINTMSG_RTOHAND as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive() (groups are values in Rust)
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else {
        return;
    };
    // Duel.SendtoHand(g,nil,REASON_COST)
    duel::send_to_hand(d, g, None, REASON_COST, None);
    // g:DeleteGroup() (groups are values in Rust)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // e:SetLabel(e:GetHandler():GetPreviousControler())
    let h = effect::get_handler(d, e).nil();
    let prev = card::get_previous_controler(d, h);
    effect::set_label(d, e, &[prev as i64]);
    // return e:GetHandler():IsPreviousLocation(LOCATION_HAND) and r&(REASON_DISCARD|REASON_EFFECT)==REASON_DISCARD|REASON_EFFECT
    card::is_previous_location(d, h, LOCATION_HAND) && (a.r & (REASON_DISCARD | REASON_EFFECT)) == (REASON_DISCARD | REASON_EFFECT)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    // if rp==tp then
    if a.rp == tp {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, e, CATEGORY_DESTROY);
    } else {
        // e:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    }
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)~=0 and rp==1-tp and tp==e:GetLabel() then
    let Some(tc) = duel::first_target(d) else {
        return;
    };
    if !card::is_relate_to_effect(d, tc, e) || duel::destroy(d, tc, REASON_EFFECT, None, None) == 0 {
        return;
    }
    if !(a.rp == 1 - tp && tp as i64 == effect::get_label(d, e)) {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // local hg=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let hg = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    // if #hg>0 then
    if !hg.is_empty() {
        // local cg=hg:RandomSelect(tp,1)
        let cg = group::random_select(d, &hg, tp, 1);
        // local cc=cg:GetFirst()
        let cc = group::get_first(d, &cg).nil();
        // Duel.ConfirmCards(tp,cc)
        duel::confirm_cards(d, tp, cc);
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and cc:IsCanBeSpecialSummoned(e,0,tp,false,false)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, cc, e, 0, tp, false, false, None, None, None)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
        {
            // Duel.SpecialSummon(cc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, cc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        } else {
            // Duel.ShuffleHand(1-tp)
            duel::shuffle_hand(d, 1 - tp);
        }
    }
}
