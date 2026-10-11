//! Cyberdark Invasion
//! サイバーダーク・インヴェイジョン
use super::prelude::*;

pub const ID: u32 = 1157683;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CYBERDARK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(65)
    effect::set_description(d, e2, 65);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then
    if a.chkc().is_some() {
        // return s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
        return eqtg(d, a);
    }
    // local b1=s.eqtg(e,tp,eg,ep,ev,re,r,rp,0)
    let b1 = eqtg(d, &with_chk(a, 0));
    // local b2=s.destg(e,tp,eg,ep,ev,re,r,rp,0)
    let b2 = destg(d, &with_chk(a, 0));
    // if chk==0 then
    if a.chk == Some(0) {
        // return b1 or b2
        return b1 || b2;
    }
    // s.select(e,tp,b1,b2)
    select(d, a, b1, b2);
    true
}
/// `e` with `chk` set and no chain-target card (the Lua calls `s.eqtg(e,tp,...,0)`).
fn with_chk(a: &EffArgs, chk: i64) -> EffArgs {
    EffArgs { chk: Some(chk), extra: Vec::new(), ..a.clone() }
}
/// `s.select(e,tp,b1,b2)`
fn select(d: &mut Duel, a: &EffArgs, b1: bool, b2: bool) {
    let tp = a.tp;
    // local op=0
    let mut op: i64 = 0;
    // if b1 and b2 then
    if b1 && b2 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))+1
        op = duel::select_option(d, tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]) as i64 + 1;
    // elseif b1 then
    } else if b1 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,0))+1
        op = duel::select_option(d, tp, true, &[aux::stringid(ID, 0)]) as i64 + 1;
    // elseif b2 then op=Duel.SelectOption(tp,aux.Stringid(id,1))+2 end
    } else if b2 {
        op = duel::select_option(d, tp, true, &[aux::stringid(ID, 1)]) as i64 + 2;
    }
    // if op==1 then
    if op == 1 {
        // --Equip 1 Dragon or Machine to your "Cyberdark"
        // e:SetCategory(CATEGORY_EQUIP)
        effect::set_category(d, a.e, CATEGORY_EQUIP);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // s.eqtg(e,tp,eg,ep,ev,re,r,rp,1)
        eqtg(d, &with_chk(a, 1));
        // e:SetOperation(s.eqop)
        effect::set_operation(d, a.e, Some(operation(eqop)));
    // elseif op==2 then
    } else if op == 2 {
        // --Destroy 1 card the opponent controls
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // s.destg(e,tp,eg,ep,ev,re,r,rp,1)
        destg(d, &with_chk(a, 1));
        // e:SetOperation(s.desop)
        effect::set_operation(d, a.e, Some(operation(desop)));
    }
}
/// `s.eqfilter1(c)`
fn eqfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_CYBERDARK) and c:IsType(TYPE_EFFECT)
    //     and Duel.IsExistingMatchingCard(s.eqfilter2,0,LOCATION_GRAVE,LOCATION_GRAVE,1,nil)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_CYBERDARK as u16])
        && card::is_type(d, c, TYPE_EFFECT)
        && duel::is_existing_matching_card(d, Some(&eqfilter2), 0, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX)
}
/// `s.eqfilter2(c)`
fn eqfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON|RACE_MACHINE) and c:IsMonster() and not c:IsForbidden()
    card::is_race(d, c, RACE_DRAGON | RACE_MACHINE) && card::is_monster(d, c) && !card::is_forbidden(d, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.eqfilter1(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && eqfilter1(d, chkc);
    }
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.eqfilter1,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, tp, ID) == 0
            && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&eqfilter1), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,s.eqfilter1,tp,LOCATION_MZONE,0,1,1,nil)
    let _g = duel::select_target(d, tp, Some(&eqfilter1), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,PLAYER_EITHER,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, PLAYER_EITHER as u8, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local ec=Duel.GetFirstTarget()
    let ec = duel::first_target(d);
    // if ec and ec:IsRelateToEffect(e) and ec:IsFaceup() then
    if let Some(ec) = ec {
        if card::is_relate_to_effect(d, ec, a.e) && card::is_faceup(d, ec) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
            // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.eqfilter2),tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil)
            let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(eqfilter2)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
            // local tc=g:GetFirst()
            let tc = group::get_first(d, &g);
            // if not tc or not Duel.Equip(tp,tc,ec,true) then return end
            let Some(tc) = tc else {
                return;
            };
            if !duel::equip(d, tp, tc, ec, true, false) {
                return;
            }
            // Equip limit
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_EQUIP_LIMIT)
            effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetValue(s.eqlimit)
            effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
            // e1:SetLabelObject(ec)
            effect::set_label_object(d, e1, Value::Card(ec));
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // Increase ATK
            // local e2=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e2 = effect::create_effect(d, h);
            // e2:SetType(EFFECT_TYPE_EQUIP)
            effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
            // e2:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
            // e2:SetValue(1000)
            effect::set_value(d, e2, 1000);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetLabelObject()
    effect::get_label_object(d, e).as_card() == Some(c)
}
/// `s.descfilter(c)`
fn descfilter(d: &mut Duel, c: CardId) -> bool {
    // local ec=c:GetEquipTarget()
    // return ec and ec:IsRace(RACE_MACHINE) and c:IsAbleToGraveAsCost()
    match card::get_equip_target(d, c) {
        None => false,
        Some(ec) => card::is_race(d, ec, RACE_MACHINE) && card::is_able_to_grave_as_cost(d, c),
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local mg=Duel.GetMatchingGroup(s.descfilter,tp,LOCATION_SZONE,0,nil)
    let mg = duel::get_matching_group(d, Some(&descfilter), tp, LOCATION_SZONE, 0, NO_EX);
    // local dg=Duel.GetMatchingGroup(nil,tp,0,LOCATION_ONFIELD,nil)
    let dg = duel::get_matching_group(d, None, tp, 0, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return Duel.GetFlagEffect(tp,id+1)==0 and #mg>0 and #dg>0 end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, tp, ID + 1) == 0 && !mg.is_empty() && !dg.is_empty();
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=mg:Select(tp,1,1,nil)
    let g = group::select(d, &mg, tp, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,1,1-tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), 1, 1 - tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,nil,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_matching_card(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
