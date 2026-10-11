//! Infernity Pawn
//! インフェルニティ・ポーン
use super::prelude::*;

pub const ID: u32 = 82434071;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_INFERNITY, SET_VOID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Give up draw phase for 1 of 2 effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PREDRAW)
    effect::set_code(d, e1, EVENT_PREDRAW);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e1, Some(cost::self_banish()));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)  -- the script defines no s.operation: nil (the target sets it)
    effect::set_operation(d, e1, None);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // During your draw phase, if you have no cards in hand
    // return tp==Duel.GetTurnPlayer() and Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0
    //     and Duel.GetDrawCount(tp)>0 and Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)==0
    let tp = a.tp;
    tp == duel::get_turn_player(d)
        && duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > 0
        && duel::get_draw_count(d, tp) > 0
        && duel::get_field_group_count(d, tp, LOCATION_HAND, 0) == 0
}
/// `s.filter(c)`: "Void" spell/trap that can be Set
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VOID) and c:IsSpellTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_VOID as u16]) && card::is_spell_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local b1=Duel.IsExistingMatchingCard(Card.IsSetCard,tp,LOCATION_DECK,0,1,nil,SET_INFERNITY)
    let b1 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_set_card(d, c, &[SET_INFERNITY as u16])), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // op is a global in the Lua; it is only read here, so a local is the same
    let op: i32;
    // if b1 and b2 then
    if b1 && b2 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
        op = duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]);
    } else if b1 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1))
        op = duel::select_option(d, tp, true, &[aux::stringid(ID, 1)]);
    } else {
        // op=Duel.SelectOption(tp,aux.Stringid(id,2))+1
        op = duel::select_option(d, tp, true, &[aux::stringid(ID, 2)]) + 1;
    }
    // if op==0 then
    if op == 0 {
        // e:SetCategory(0)
        effect::set_category(d, e, 0);
        // e:SetOperation(s.topop)
        effect::set_operation(d, e, Some(operation(topop)));
        // s.toptg(e,tp,eg,ep,ev,re,r,rp,1)
        let mut a1 = a.clone();
        a1.chk = Some(1);
        toptg(d, &a1);
    } else {
        // e:SetCategory(CATEGORY_SET)
        effect::set_category(d, e, CATEGORY_SET);
        // e:SetOperation(s.setop)
        effect::set_operation(d, e, Some(operation(setop)));
        // s.settg(e,tp,eg,ep,ev,re,r,rp,1)
        let mut a1 = a.clone();
        a1.chk = Some(1);
        settg(d, &a1);
    }
    // local dt=Duel.GetDrawCount(tp)
    let dt = duel::get_draw_count(d, tp);
    // if dt~=0 then
    if dt != 0 {
        // _replace_count=0
        set_lua_global(d, "_replace_count", Value::Int(0));
        // _replace_max=dt
        set_lua_global(d, "_replace_max", Value::Int(dt as i64));
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetCode(EFFECT_DRAW_COUNT)
        effect::set_code(d, e1, EFFECT_DRAW_COUNT);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetReset(RESET_PHASE|PHASE_DRAW)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW, 1);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
    true
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`: Set 1 "Void" spell/trap from deck
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // _replace_count=_replace_count+1
    let rc = lua_global(d, "_replace_count");
    if rc.is_nil() {
        crate::lua_error("attempt to perform arithmetic on a nil value");
    }
    set_lua_global(d, "_replace_count", Value::Int(rc.int() + 1));
    // if _replace_count<=_replace_max then
    if lua_global(d, "_replace_count").int() <= lua_global(d, "_replace_max").int() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
        // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SSet(tp,g:GetFirst())
            let first = group::get_first(d, &g).nil();
            duel::s_set(d, tp, first, None, true);
        }
    }
}
/// `s.toptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn toptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsSetCard,tp,LOCATION_DECK,0,1,nil,SET_INFERNITY) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_set_card(d, c, &[SET_INFERNITY as u16])), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.topop(e,tp,eg,ep,ev,re,r,rp)`: Place 1 "Infernity" monster on top of deck
fn topop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // _replace_count=_replace_count+1
    let rc = lua_global(d, "_replace_count");
    if rc.is_nil() {
        crate::lua_error("attempt to perform arithmetic on a nil value");
    }
    set_lua_global(d, "_replace_count", Value::Int(rc.int() + 1));
    // if _replace_count<=_replace_max then
    if lua_global(d, "_replace_count").int() <= lua_global(d, "_replace_max").int() {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 3));
        // local g=Duel.SelectMatchingCard(tp,Card.IsSetCard,tp,LOCATION_DECK,0,1,1,nil,SET_INFERNITY)
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&|d, c| card::is_set_card(d, c, &[SET_INFERNITY as u16])),
            tp,
            LOCATION_DECK,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // local tc=g:GetFirst()
        let tc = group::get_first(d, &g);
        // if tc then
        if let Some(tc) = tc {
            // Duel.ShuffleDeck(tp)
            duel::shuffle_deck(d, tp);
            // Duel.MoveSequence(tc,0)
            duel::move_sequence(d, tc, 0, None);
            // Duel.ConfirmDecktop(tp,1)
            duel::confirm_decktop(d, tp, 1);
        }
    }
}
