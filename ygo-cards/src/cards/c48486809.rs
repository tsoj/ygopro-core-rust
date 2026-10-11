//! Winged Kuriboh LV6
use super::prelude::*;

pub const ID: u32 = 48486809;

pub const INFO: ScriptInfo = ScriptInfo {
    lv: Some((6, SET_WINGED_KURIBOH)),
    listed_names: &[57116033],
    listed_series: &[SET_ELEMENTAL_HERO, SET_FAVORITE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Must be Special Summoned with its own procedure
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon Procedure
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_HAND | LOCATION_GRAVE);
    // e2:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e2, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(lua_fn(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy an opponent's monster that declares an attack and inflict damage equal to its original ATK
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end)
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    // e3:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e3, Some(cost::self_tribute()));
    // e3:SetTarget(s.destg1)
    effect::set_target(d, e3, Some(cost_target(destg1)));
    // e3:SetOperation(s.desop1)
    effect::set_operation(d, e3, Some(operation(desop1)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy an opponent's monster that activates an effect on the field and inflict damage equal to its original ATK
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetCondition(s.descond)
    effect::set_condition(d, e4, Some(condition(descond)));
    // e4:SetTarget(s.destg2)
    effect::set_target(d, e4, Some(cost_target(destg2)));
    // e4:SetOperation(s.desop2)
    effect::set_operation(d, e4, Some(operation(desop2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.spcfilter(c)`
fn spcfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsCode(57116033) or (c:IsType(TYPE_FUSION) and c:IsSetCard(SET_ELEMENTAL_HERO)))
    //     and c:IsMonster() and (c:IsFaceup() or c:IsLocation(LOCATION_HAND))
    //     and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true,true)
    (card::is_code(d, c, &[57116033]) || (card::is_type(d, c, TYPE_FUSION) && card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16])))
        && card::is_monster(d, c)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, true)
}
/// `s.spcon(e,c)` (condition of the Special Summon procedure)
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    let e = a.e(0);
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local rg=Duel.GetMatchingGroup(s.spcfilter,tp,LOCATION_MZONE|LOCATION_HAND|LOCATION_GRAVE,0,e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let rg = duel::get_matching_group(d, Some(&spcfilter), tp, LOCATION_MZONE | LOCATION_HAND | LOCATION_GRAVE, 0, h.into());
    // return #rg>0 and aux.SelectUnselectGroup(rg,e,tp,1,1,aux.ChkfMMZ(1),0)
    !rg.is_empty()
        && matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), Some(&aux::chkf_mmz(1)), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)` (target of the Special Summon procedure)
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetMatchingGroup(s.spcfilter,tp,LOCATION_MZONE|LOCATION_HAND|LOCATION_GRAVE,0,e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let rg = duel::get_matching_group(d, Some(&spcfilter), tp, LOCATION_MZONE | LOCATION_HAND | LOCATION_GRAVE, 0, h.into());
    // local g=aux.SelectUnselectGroup(rg,e,tp,1,1,aux.ChkfMMZ(1),1,tp,HINTMSG_REMOVE,nil,nil,true)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), Some(&aux::chkf_mmz(1)), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
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
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)` (operation of the Special Summon procedure)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned();
    // if not g then return end
    let Some(g) = g else {
        return;
    };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup()  (groups are values: nothing to do)
}
/// `function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end` (condition of e3)
fn e3_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let at = duel::get_attacker(d).nil();
    card::is_controler(d, at, 1 - a.tp)
}
/// `s.destg1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg1(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // if chk==0 then return at:IsRelateToBattle() end
    if a.chk == Some(0) {
        return card::is_relate_to_battle(d, at);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,at,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(at)), 1, a.tp, 0);
    // if at:GetBaseAttack()>0 then
    if card::get_base_attack(d, at) > 0 {
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,1,1-tp,at:GetBaseAttack())
        let atk = card::get_base_attack(d, at);
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 1, 1 - a.tp, atk);
    }
    true
}
/// `s.desop1(e,tp,eg,ep,ev,re,r,rp)`
fn desop1(d: &mut Duel, a: &EffArgs) {
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // if not at:IsRelateToBattle() or Duel.Destroy(at,REASON_EFFECT)==0 then return end
    if !card::is_relate_to_battle(d, at) || duel::destroy(d, at, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // local atk=at:GetBaseAttack()
    let atk = card::get_base_attack(d, at);
    // if atk>0 then
    if atk > 0 {
        // Duel.Damage(1-tp,atk,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, atk as i64, REASON_EFFECT, false, None);
    }
}
/// `s.descond(e,tp,eg,ep,ev,re,r,rp)`
fn descond(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:IsMonsterEffect() and re:GetActivateLocation()==LOCATION_MZONE
    if a.rp != 1 - a.tp {
        return false;
    }
    let re = a.re.nil();
    effect::is_monster_effect(d, re) && effect::get_activate_location(d, re) == LOCATION_MZONE
}
/// `s.destg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg2(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // if chk==0 then return rc:IsDestructable() and rc:IsRelateToEffect(re) end
    if a.chk == Some(0) {
        return card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,rc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(rc)), 1, a.tp, 0);
    // if rc:GetBaseAttack()>0 then
    if card::get_base_attack(d, rc) > 0 {
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,1,1-tp,rc:GetBaseAttack())
        let atk = card::get_base_attack(d, rc);
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 1, 1 - a.tp, atk);
    }
    true
}
/// `s.desop2(e,tp,eg,ep,ev,re,r,rp)`
fn desop2(d: &mut Duel, a: &EffArgs) {
    // local rc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let rc = group::get_first(d, &eg).nil();
    let re = a.re.nil();
    // if not rc:IsRelateToEffect(re) or Duel.Destroy(rc,REASON_EFFECT)==0 then return end
    if !card::is_relate_to_effect(d, rc, re) || duel::destroy(d, rc, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // local atk=rc:GetBaseAttack()
    let atk = card::get_base_attack(d, rc);
    // if atk>0 then
    if atk > 0 {
        // Duel.Damage(1-tp,atk,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, atk as i64, REASON_EFFECT, false, None);
    }
}
