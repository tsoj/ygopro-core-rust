//! Guilt-Gripping Morganite
use super::prelude::*;

pub const ID: u32 = 61822419;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MORGANITE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply effects for the rest of the Duel
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place 1 revealed "Morganite" card on the bottom of the Deck, then draw 1 card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.tddrawcost)
    effect::set_cost(d, e2, Some(cost_target(tddrawcost)));
    // e2:SetTarget(s.tddrawtg)
    effect::set_target(d, e2, Some(cost_target(tddrawtg)));
    // e2:SetOperation(s.tddrawop)
    effect::set_operation(d, e2, Some(operation(tddrawop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not Duel.HasFlagEffect(tp,id) end
    if a.chk == Some(0) {
        return !duel::has_flag_effect(d, a.tp, ID, None);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.HasFlagEffect(tp,id) then return end
    if duel::has_flag_effect(d, tp, ID, None) {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id,0,0,1)
    duel::register_flag_effect(d, tp, ID, 0, 0, 1, 0);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // You cannot activate monster effects in the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(s.aclimit)
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // You can Normal Summon Level 5 or higher monsters without Tributing.
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e2, aux::stringid(ID, 3));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SUMMON_PROC);
    // e2:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e2, LOCATION_HAND, 0);
    // e2:SetCondition(s.nontribcon)
    effect::set_condition(d, e2, Some(lua_fn(nontribcon)));
    // e2:SetTarget(aux.FieldSummonProcTg(function(e,c) return c:IsLevelAbove(5) end))
    effect::set_target(d, e2, Some(aux::field_summon_proc_tg(Some(effect_card_fn(|d, _e, c| card::is_level_above(d, c, 5))), None)));
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // You do not pay LP to activate Spell/Trap Cards
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_LPCOST_CHANGE)
    effect::set_code(d, e3, EFFECT_LPCOST_CHANGE);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // e3:SetValue(s.nolpcost)
    effect::set_value_fn(d, e3, lua_fn(nolpcost));
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, tp);
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let re = a.e(1);
    let Some(rc) = effect::get_handler(d, re) else {
        return false;
    };
    // return rc and rc:IsLocation(LOCATION_HAND) and re:IsMonsterEffect()
    card::is_location(d, rc, LOCATION_HAND) && effect::is_monster_effect(d, re)
}
/// `s.nontribcon(e,c,minc)`
fn nontribcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return minc==0 and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    let tp = card::get_controler(d, c);
    matches!(a.v(2), Value::Int(0)) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.nolpcost(e,re,rp,val)`
fn nolpcost(d: &mut Duel, a: Args) -> i64 {
    // return (re and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():IsSpellTrap()) and 0 or val
    let x = match a.v(1).as_effect() {
        Some(re) => {
            effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && {
                let rc = effect::get_handler(d, re).nil();
                card::is_spell_trap(d, rc)
            }
        }
        None => false,
    };
    if x { 0 } else { a.int(3) }
}
/// `s.revealfilter(c)`
fn revealfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MORGANITE) and not c:IsPublic()
    card::is_set_card(d, c, &[SET_MORGANITE as u16]) && !card::is_public(d, c)
}
/// `s.tddrawcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tddrawcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost() and Duel.IsExistingMatchingCard(s.revealfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None) && duel::is_existing_matching_card(d, Some(&revealfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Remove(c,POS_FACEUP,REASON_COST)
    duel::remove(d, c, POS_FACEUP as u8, REASON_COST, None, None);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local rc=Duel.SelectMatchingCard(tp,s.revealfilter,tp,LOCATION_HAND,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&revealfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let rc = group::get_first(d, &g).nil();
    // Duel.ConfirmCards(1-tp,rc)
    duel::confirm_cards(d, 1 - a.tp, rc);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    // e:SetLabelObject(rc)
    effect::set_label_object(d, a.e, Value::Card(rc));
    // Duel.SetTargetCard(rc)
    duel::set_target_card(d, rc);
    true
}
/// `s.tddrawtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tddrawtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,e:GetLabelObject(),1,tp,0)
    let rc = effect::get_label_object(d, a.e).as_card().map(CardSet::from_card);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, rc, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.tddrawop(e,tp,eg,ep,ev,re,r,rp)`
fn tddrawop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local rc=e:GetLabelObject()
    let rc = effect::get_label_object(d, e).as_card().nil();
    // if rc:IsRelateToEffect(e) and Duel.SendtoDeck(rc,nil,SEQ_DECKBOTTOM,REASON_EFFECT)>0 and Duel.IsPlayerCanDraw(tp) then
    if card::is_relate_to_effect(d, rc, e)
        && duel::send_to_deck(d, rc, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0
        && duel::is_player_can_draw(d, tp, 0)
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
