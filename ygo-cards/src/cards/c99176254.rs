//! Maiden of Blue Tears
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 99176254;

/// `s.sumgroup`: the monsters Special Summoned, kept as a variable of the duel.
const SUMGROUP: u64 = 0;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 monster Special Summoned by your opponent and inflict damage to your opponent equal to half its original ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e1, EVENT_CUSTOM + ID);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Keep track of all monsters that are Special Summoned
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // s.sumgroup=Group.CreateGroup()
        // s.sumgroup:KeepAlive()
        set_s_var(d, ID, SUMGROUP, Value::Group(CardSet::new()));
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, ge1, EVENT_SPSUMMON_SUCCESS);
        // ge1:SetOperation(s.sumgreg)
        effect::set_operation(d, ge1, Some(operation(sumgreg)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
    // Set 1 Normal Spell from your GY or banishment
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_DAMAGE)
    effect::set_code(d, e2, EVENT_DAMAGE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return (r&REASON_EFFECT)>0 end)
    effect::set_condition(d, e2, Some(condition(e2cond)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// The monsters Special Summoned so far (`s.sumgroup`).
fn sumgroup(d: &Duel) -> CardSet {
    s_var(d, ID, SUMGROUP).as_group().cloned().unwrap_or_default()
}
/// `s.sumgreg(e,tp,eg,ep,ev,re,r,rp)`
fn sumgreg(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg();
    // for tc in eg:Iter() do tc:RegisterFlagEffect(id,RESET_CHAIN,0,1) end
    for tc in group::iter(d, &eg) {
        card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
    }
    let mut sg = sumgroup(d);
    // if Duel.GetCurrentChain()==0 then s.sumgroup:Clear() end
    if duel::get_current_chain(d, false) == 0 {
        sg = CardSet::new();
    }
    // s.sumgroup:Merge(eg)
    group::add_card(&mut sg, &eg);
    // s.sumgroup:Remove(function(c) return not c:HasFlagEffect(id) end,nil)
    group::remove(d, &mut sg, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
    set_s_var(d, ID, SUMGROUP, Value::Group(sg.clone()));
    // Duel.RaiseEvent(s.sumgroup,EVENT_CUSTOM+id,e,0,tp,tp,0)
    duel::raise_event(d, sg, EVENT_CUSTOM + ID, Some(a.e), 0, a.tp, a.tp, 0);
}
/// `s.desfilter(c,tp,e)`
fn desfilter(d: &mut Duel, c: CardId, tp: u8, e: EffectId) -> bool {
    // return c:IsSummonPlayer(1-tp) and c:IsLocation(LOCATION_MZONE) and c:IsCanBeEffectTarget(e)
    card::is_summon_player(d, c, 1 - tp) && card::is_location(d, c, LOCATION_MZONE) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.destg(e,tp,...,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local g=s.sumgroup:Filter(s.desfilter,nil,tp,e)
    let sg = sumgroup(d);
    let g = group::filter(d, &sg, &|d, c| desfilter(d, c, tp, e), NO_EX);
    // if chkc then return g:IsContains(chkc) and s.desfilter(chkc,tp,e) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && desfilter(d, chkc, tp, e);
    }
    // if chk==0 then return #g>0 and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsLinkMonster),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return !g.is_empty()
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_link_monster(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // local tc=nil
    // if #g==1 then tc=g:GetFirst()
    // else Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY) tc=g:Select(tp,1,1,nil):GetFirst() end
    let tc = if g.len() == 1 {
        group::get_first(d, &g).nil()
    } else {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        group::get_first(d, &sel).nil()
    };
    // Duel.SetTargetCard(tc)
    duel::set_target_card(d, tc);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(tc)), 1, tp, 0);
    // if tc:IsFaceup() and tc:GetBaseAttack()>0 then
    if card::is_faceup(d, tc) && card::get_base_attack(d, tc) > 0 {
        let base = card::get_base_attack(d, tc);
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,1,1-tp,tc:GetBaseAttack()//2)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 1, 1 - tp, base.div_euclid(2));
    } else {
        // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,1,1-tp,0)
        duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 1, 1 - tp, 0);
    }
    true
}
/// `s.desop(e,tp,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 and tc:GetBaseAttack()>0 then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None) > 0
        && card::get_base_attack(d, tc) > 0
    {
        let base = card::get_base_attack(d, tc);
        // Duel.Damage(1-tp,tc:GetBaseAttack()//2,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, base.div_euclid(2) as i64, REASON_EFFECT, false, None);
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return (r&REASON_EFFECT)>0 end`
fn e2cond(_d: &mut Duel, a: &EffArgs) -> bool {
    (a.r & REASON_EFFECT) > 0
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNormalSpell() and c:IsFaceup() and c:IsSSetable()
    card::is_normal_spell(d, c) && card::is_faceup(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,...,chk,chkc)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) and s.setfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED) && setfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.setfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&setfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local tc=Duel.SelectTarget(tp,s.setfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil):GetFirst()
    let sel = duel::select_target(d, a.tp, Some(&setfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // if tc:IsLocation(LOCATION_GRAVE) then Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,tc,1,tp,0) end
    if card::is_location(d, tc, LOCATION_GRAVE) {
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(tc)), 1, a.tp, 0);
    }
    true
}
/// `s.setop(e,tp,...)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SSet(tp,tc)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::s_set(d, a.tp, CardSet::from_card(tc), None, true) > 0 {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        // e1:SetReset(RESET_EVENT|RESETS_CANNOT_ACT|RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_CANNOT_ACT | RESET_PHASE | PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
