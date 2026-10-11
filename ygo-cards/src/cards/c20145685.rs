//! Xyz Armor Fortress
use super::prelude::*;

pub const ID: u32 = 20145685;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_ARMORED_XYZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 5 monsters OR 1 Rank 3 or 4 Xyz Monster you control
    // Xyz.AddProcedure(c,nil,5,2,s.xyzfilter,aux.Stringid(id,0),2,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(5),
        2,
        Some(xyz::alter_filter(xyzfilter)),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::N(2)),
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // Cannot be used as material for an Xyz Summon while it has material
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_CANNOT_BE_XYZ_MATERIAL)
    effect::set_code(d, e0, EFFECT_CANNOT_BE_XYZ_MATERIAL);
    // e0:SetCondition(function(e) return e:GetHandler():GetOverlayCount()>0 end)
    effect::set_condition(d, e0, Some(lua_fn(cond0)));
    // e0:SetValue(1)
    effect::set_value(d, e0, 1);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Add "Armored Xyz" cards with different names from your Deck to your hand, equal to the number of materials detached
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1,s.thmaxcost,function(e,og) e:SetLabel(#og) end))
    effect::set_cost(d, e1, Some(cost::detach_from_self(
        cost::Count::N(1),
        Some(cost::Count::F(std::sync::Arc::new(thmaxcost))),
        Some(std::sync::Arc::new(set_label_og)),
    )));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If a monster equipped with this card battles a monster, any battle damage it inflicts to your opponent is doubled
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e2:SetCondition(function(e) return e:GetHandler():GetEquipTarget():GetBattleTarget() end)
    effect::set_condition(d, e2, Some(lua_fn(cond2)));
    // e2:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
    effect::set_value_fn(d, e2, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.xyzfilter(c,tp,xyzc)
fn xyzfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // return c:IsRank(3,4) and c:IsType(TYPE_XYZ,xyzc,SUMMON_TYPE_XYZ,tp) and c:IsFaceup()
    card::is_rank(d, c, &[3, 4])
        && card::is_type_ctx(d, c, TYPE_XYZ, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_faceup(d, c)
}
// s.xyzop(e,tp,chk)
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let (tp, chk) = (a.p(1), a.int(2));
    // if chk==0 then return not Duel.HasFlagEffect(tp,id) end
    if chk == 0 {
        return !duel::has_flag_effect(d, tp, ID, None);
    }
    // return Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,EFFECT_FLAG_OATH,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, EFFECT_FLAG_OATH, 1, 0).is_some()
}
// s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ARMORED_XYZ) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ARMORED_XYZ as u16]) && card::is_able_to_hand(d, c, None)
}
// s.thmaxcost(e,tp)
fn thmaxcost(d: &mut Duel, _e: EffectId, tp: u8) -> i64 {
    // return math.min(2,Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil):GetClassCount(Card.GetCode))
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    let n = group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) as i64;
    2.min(n)
}
// function(e,og) e:SetLabel(#og) end
fn set_label_og(d: &mut Duel, e: EffectId, og: &CardSet) {
    effect::set_label(d, e, &[og.len() as i64]);
}
// s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,e:GetLabel(),tp,LOCATION_DECK)
    let label = effect::get_label(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, label as u8, a.tp, LOCATION_DECK as i32);
    true
}
// s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e) as usize;
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g<ct then return end
    if g.len() < ct {
        return;
    }
    // local thg=aux.SelectUnselectGroup(g,e,tp,ct,ct,aux.dncheck,1,tp,HINTMSG_ATOHAND)
    let thg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(ct), Some(ct), Some(&aux::dncheck()), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false)
        .unwrap_or_default();
    // if #thg>0 then
    if !thg.is_empty() {
        // Duel.SendtoHand(thg,nil,REASON_EFFECT)
        duel::send_to_hand(d, &thg, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,thg)
        duel::confirm_cards(d, 1 - tp, &thg);
    }
}
// s.cond0: function(e) return e:GetHandler():GetOverlayCount()>0 end
fn cond0(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_overlay_count(d, h) > 0
}
// s.cond2: function(e) return e:GetHandler():GetEquipTarget():GetBattleTarget() end
fn cond2(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    let et = card::get_equip_target(d, h).nil();
    card::get_battle_target(d, et).is_some()
}
