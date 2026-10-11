//! Orphebull the Harmonious Bullfighter Bard
//! Scripted by fiftyfour
use super::prelude::*;

pub const ID: u32 = 97240499;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Must be Special Summoned by own procedure
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.FALSE)
    // (the Lua never calls c:RegisterEffect(e1): the effect is only created)
    effect::set_value_fn(d, e1, aux_false());
    // Special Summon by banishing 2 LIGHT and 2 DARK monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
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
    // Banish destroyed monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLED)
    effect::set_code(d, e3, EVENT_BATTLED);
    // e3:SetCondition(s.rmcon)
    effect::set_condition(d, e3, Some(condition(rmcon)));
    // e3:SetTarget(s.rmtg)
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.spfilter(c)
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && card::is_attribute(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK)
        && ((card::is_location(d, c, LOCATION_GRAVE | LOCATION_MZONE) && aux::sp_elim_filter(d, c, true, false))
            || card::is_location(d, c, LOCATION_HAND))
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.attchk,2,nil,sg)
    let (ok, _) = aux::chkf_mmz(1)(d, sg, e, tp, mg, None);
    (ok && group::is_exists(d, sg, &|d, c| attchk(d, c, sg), 2, NO_EX), false)
}
// function s.attchk(c,sg)
fn attchk(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && group::filter_count(d, sg, &|d, x| card::is_attribute(d, x, ATTRIBUTE_DARK), c.into()) == 2
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE|LOCATION_HAND,0,e:GetHandler())
    let h = effect::get_handler(d, a.e(0)).nil();
    let rg = duel::get_matching_group(
        d,
        Some(&|d, x| spfilter(d, x)),
        tp,
        LOCATION_MZONE | LOCATION_GRAVE | LOCATION_HAND,
        0,
        h.into(),
    );
    // return #rg>=4 and aux.SelectUnselectGroup(rg,e,tp,4,4,s.rescon,0)
    rg.len() >= 4
        && matches!(
            aux::select_unselect_group(d, &rg, Some(a.e(0)), tp, Some(4), Some(4), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        )
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,c)
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE|LOCATION_HAND,0,e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let rg = duel::get_matching_group(
        d,
        Some(&|d, x| spfilter(d, x)),
        tp,
        LOCATION_MZONE | LOCATION_GRAVE | LOCATION_HAND,
        0,
        h.into(),
    );
    // local g=aux.SelectUnselectGroup(rg,e,tp,4,4, s.rescon,1,tp,HINTMSG_REMOVE)
    let g = aux::select_unselect_group(
        d,
        &rg,
        Some(e),
        tp,
        Some(4),
        Some(4),
        Some(&rescon),
        1,
        tp,
        Some(HINTMSG_REMOVE as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp,c)
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let tp = a.p(1);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else { return };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST, None, None);
    // local hc=g:FilterCount(Card.IsPreviousLocation,nil,LOCATION_HAND)
    let hc = group::filter_count(d, &g, &|d, x| card::is_previous_location(d, x, LOCATION_HAND), NO_EX);
    // local gc=g:FilterCount(Card.IsPreviousLocation,nil,LOCATION_GRAVE)
    let gc = group::filter_count(d, &g, &|d, x| card::is_previous_location(d, x, LOCATION_GRAVE), NO_EX);
    // if hc>0 then
    if hc > 0 {
        let c = a.c(8);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(hc*1000)
        effect::set_value(d, e1, hc as i64 * 1000);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TOFIELD)
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // if gc>0 then
    if gc > 0 {
        let c = a.c(8);
        // local e2 = Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_EXTRA_ATTACK_MONSTER)
        effect::set_code(d, e2, EFFECT_EXTRA_ATTACK_MONSTER);
        // e2:SetValue(gc-1)
        effect::set_value(d, e2, gc as i64 - 1);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TOFIELD)
        effect::set_reset(d, e2, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
    // g:DeleteGroup()  (groups are values: nothing to do)
    let _ = tp;
}
// function s.rmcon(e,tp,eg,ep,ev,re,r,rp)
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsStatus(STATUS_BATTLE_DESTROYED) then return false end
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // if bc and bc:IsControler(1-tp) and bc:IsStatus(STATUS_BATTLE_DESTROYED) then
    if let Some(bc) = bc {
        if card::is_controler(d, bc, 1 - a.tp) && card::is_status(d, bc, STATUS_BATTLE_DESTROYED) {
            // e:SetLabelObject(bc)
            effect::set_label_object(d, a.e, Value::Card(bc));
            // return true
            return true;
        }
    }
    // return false
    false
}
// function s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetLabelObject()
    let bc = effect::get_label_object(d, a.e).as_card();
    // if chk==0 then return bc:IsAbleToRemove() end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, bc.nil(), None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,bc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, bc.map(CardSet::from_card), 1, 0, 0);
    true
}
// function s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetLabelObject()
    let bc = effect::get_label_object(d, a.e).as_card().nil();
    // if bc:IsRelateToBattle() and bc:IsControler(1-tp) then
    if card::is_relate_to_battle(d, bc) && card::is_controler(d, bc, 1 - a.tp) {
        // Duel.Remove(bc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, bc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
