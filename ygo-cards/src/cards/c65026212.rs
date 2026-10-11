//! Koa'ki Meiru Maximus
use super::prelude::*;

pub const ID: u32 = 65026212;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KOAKI_MEIRU], listed_names: &[36623431], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // cost
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.mtcon)
    effect::set_condition(d, e1, Some(condition(mtcon)));
    // e1:SetOperation(s.mtop)
    effect::set_operation(d, e1, Some(operation(mtop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // spsummon proc
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(lua_fn(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e3, aux::stringid(ID, 3));
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.mtcon(e,tp,eg,ep,ev,re,r,rp)`
fn mtcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.cfilter1(c)`
fn cfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(36623431) and c:IsAbleToGraveAsCost()
    card::is_code(d, c, &[36623431]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.cfilter2(c)`
fn cfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_KOAKI_MEIRU) and c:IsAbleToGraveAsCost()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_KOAKI_MEIRU as u16]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.mtop(e,tp,eg,ep,ev,re,r,rp)`
fn mtop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.HintSelection(Group.FromCards(c))
    duel::hint_selection(d, CardSet::from_card(c), true);
    // local g1=Duel.GetMatchingGroup(s.cfilter1,tp,LOCATION_HAND,0,nil)
    let g1 = duel::get_matching_group(d, Some(&cfilter1), tp, LOCATION_HAND, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.cfilter2,tp,LOCATION_HAND,0,nil)
    let g2 = duel::get_matching_group(d, Some(&cfilter2), tp, LOCATION_HAND, 0, NO_EX);
    // local select=2  (every branch below sets select, so the initial 2 is never read)
    let mut select: i32;
    // if #g1>0 and #g2>0 then
    if !g1.is_empty() && !g2.is_empty() {
        // select=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1),aux.Stringid(id,2))
        select = duel::select_option(d, tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1), aux::stringid(ID, 2)]);
    // elseif #g1>0 then
    } else if !g1.is_empty() {
        // select=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,2))
        select = duel::select_option(d, tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 2)]);
        // if select==1 then select=2 end
        if select == 1 {
            select = 2;
        }
    // elseif #g2>0 then
    } else if !g2.is_empty() {
        // select=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))+1
        select = duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]) + 1;
    } else {
        // select=Duel.SelectOption(tp,aux.Stringid(id,2))
        let _ = duel::select_option(d, tp, true, &[aux::stringid(ID, 2)]);
        // select=2
        select = 2;
    }
    // if select==0 then
    if select == 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=g1:Select(tp,1,1,nil)
        let g = group::select(d, &g1, tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoGrave(g,REASON_COST)
        duel::send_to_grave(d, g, REASON_COST, None, None);
    // elseif select==1 then
    } else if select == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=g2:Select(tp,1,1,nil)
        let g = group::select(d, &g2, tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoGrave(g,REASON_COST)
        duel::send_to_grave(d, g, REASON_COST, None, None);
    } else {
        // Duel.Destroy(c,REASON_COST)
        duel::destroy(d, c, REASON_COST, None, None);
    }
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(36623431) and c:IsAbleToRemoveAsCost()
    card::is_code(d, c, &[36623431]) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    let e = a.e(0);
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local rg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_HAND,0,nil)
    let rg = duel::get_matching_group(d, Some(&filter), tp, LOCATION_HAND, 0, NO_EX);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and #rg>0 and aux.SelectUnselectGroup(rg,e,tp,1,1,nil,0)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && !rg.is_empty()
        && matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), None, 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, e);
    // local g=nil
    // local rg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_HAND,0,nil,tp)
    let rg = duel::get_matching_group(d, Some(&filter), tp, LOCATION_HAND, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,1,1,nil,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), None, 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
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
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.TRUE,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
