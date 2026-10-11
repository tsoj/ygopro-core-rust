//! Superheavy Samurai Thief
//! 超重武者ヌス－10
use super::prelude::*;

pub const ID: u32 = 14756848;

// s.listed_series={SET_SUPERHEAVY_SAMURAI}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUPERHEAVY_SAMURAI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.hspcon)
    effect::set_condition(d, e1, Some(lua_fn(hspcon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // spsummon limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.hspcon2)
    effect::set_condition(d, e2, Some(condition(hspcon2)));
    // e2:SetOperation(s.hspop)
    effect::set_operation(d, e2, Some(operation(hspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e3, Some(cost::self_tribute()));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.hspcon(e,c)`
fn hspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and not Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,LOCATION_GRAVE,0,1,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && !duel::is_existing_matching_card(d, Some(&card::is_spell_trap), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.hspcon2(e,tp,eg,ep,ev,re,r,rp)`
fn hspcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_SPECIAL+1
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, h) == SUMMON_TYPE_SPECIAL + 1
}
/// `s.hspop(e,tp,eg,ep,ev,re,r,rp)`
fn hspop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetDescription(aux.Stringid(id,5))
    effect::set_description(d, e1, aux::stringid(ID, 5));
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_SUPERHEAVY_SAMURAI)
    !card::is_set_card(d, c, &[SET_SUPERHEAVY_SAMURAI as u16])
}
/// `s.desfilter1(c)`
fn desfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:GetSequence()<5
    card::get_sequence(d, c) < 5
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local sel=0
        let mut sel: i64 = 0;
        // if Duel.IsExistingMatchingCard(s.desfilter1,tp,0,LOCATION_SZONE,1,nil) then sel=sel+1 end
        if duel::is_existing_matching_card(d, Some(&desfilter1), tp, 0, LOCATION_SZONE, 1, NO_EX) {
            sel += 1;
        }
        // if Duel.GetFieldGroupCount(tp,0,LOCATION_PZONE)>0 then sel=sel+2 end
        if duel::get_field_group_count(d, tp, 0, LOCATION_PZONE) > 0 {
            sel += 2;
        }
        // e:SetLabel(sel)
        effect::set_label(d, a.e, &[sel]);
        // return sel~=0
        return sel != 0;
    }
    // local sel=e:GetLabel()
    let mut sel = effect::get_label(d, a.e);
    // if sel==3 then
    if sel == 3 {
        // sel=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))+1
        sel = duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]) as i64 + 1;
    } else if sel == 1 {
        // Duel.SelectOption(tp,aux.Stringid(id,1))
        duel::select_option(d, tp, true, &[aux::stringid(ID, 1)]);
    } else {
        // Duel.SelectOption(tp,aux.Stringid(id,2))
        duel::select_option(d, tp, true, &[aux::stringid(ID, 2)]);
    }
    // e:SetLabel(sel)
    effect::set_label(d, a.e, &[sel]);
    // if sel==1 then
    if sel == 1 {
        // e:SetCategory(CATEGORY_DESTROY+CATEGORY_SET)
        effect::set_category(d, a.e, CATEGORY_DESTROY + CATEGORY_SET);
        // local g=Duel.GetMatchingGroup(s.desfilter1,tp,0,LOCATION_SZONE,nil)
        let g = duel::get_matching_group(d, Some(&desfilter1), tp, 0, LOCATION_SZONE, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    } else {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        // local g=Duel.GetFieldGroup(tp,0,LOCATION_PZONE)
        let g = duel::get_field_group(d, tp, 0, LOCATION_PZONE);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    }
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local sel=e:GetLabel()
    let sel = effect::get_label(d, a.e);
    // if sel==1 then
    if sel == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,s.desfilter1,tp,0,LOCATION_SZONE,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&desfilter1), tp, 0, LOCATION_SZONE, 1, 1, false, NO_EX).nil();
        // local tc=g:GetFirst()
        // if not tc then return end
        let Some(tc) = group::get_first(d, &g) else { return };
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // if Duel.Destroy(g,REASON_EFFECT)~=0 and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
        //     and not tc:IsLocation(LOCATION_HAND|LOCATION_DECK)
        //     and tc:IsSpellTrap() and tc:IsSSetable()
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
        if duel::destroy(d, g.clone(), REASON_EFFECT, None, None) != 0
            && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && !card::is_location(d, tc, LOCATION_HAND | LOCATION_DECK)
            && card::is_spell_trap(d, tc)
            && card::is_s_setable(d, tc, false, None)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SSet(tp,tc)
            duel::s_set(d, tp, tc, None, true);
        }
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.GetFieldGroup(tp,0,LOCATION_PZONE):Select(tp,1,1,nil)
        let pz = duel::get_field_group(d, tp, 0, LOCATION_PZONE);
        let g = group::select(d, &pz, tp, 1, 1, false, NO_EX).nil();
        // local tc=g:GetFirst()
        // if not tc then return end
        let Some(tc) = group::get_first(d, &g) else { return };
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // if Duel.Destroy(g,REASON_EFFECT)~=0
        //     and Duel.CheckPendulumZones(tp)
        //     and not tc:IsLocation(LOCATION_HAND|LOCATION_DECK) and not tc:IsForbidden()
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,4)) then
        if duel::destroy(d, g.clone(), REASON_EFFECT, None, None) != 0
            && duel::check_pendulum_zones(d, tp)
            && !card::is_location(d, tc, LOCATION_HAND | LOCATION_DECK)
            && !card::is_forbidden(d, tc)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 4))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.MoveToField(tc,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
            duel::move_to_field(d, tc, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
        }
    }
}
